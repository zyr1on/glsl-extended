// glsl_validator - analyzer_bridge.rs
// Manages glsl_analyzer child process via stdio JSON-RPC, multiplexing with glslangValidator.

use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::log;

pub struct AnalyzerBridge {
    stdin: Mutex<ChildStdin>,
    child: Mutex<Child>,
    pending_requests: Arc<Mutex<HashMap<u64, mpsc::Sender<Value>>>>,
    next_id: AtomicU64,
    is_alive: Arc<AtomicBool>,
}

impl AnalyzerBridge {
    pub fn start(binary_path: &str) -> Option<Self> {
        let mut cmd = crate::create_command(binary_path);
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());

        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                log(&format!("Failed to spawn glsl_analyzer at '{binary_path}': {e}"));
                return None;
            }
        };

        let stdin = child.stdin.take()?;
        let stdout = child.stdout.take()?;

        let pending_requests = Arc::new(Mutex::new(HashMap::<u64, mpsc::Sender<Value>>::new()));
        let pending_for_reader = Arc::clone(&pending_requests);
        let is_alive = Arc::new(AtomicBool::new(true));
        let is_alive_reader = Arc::clone(&is_alive);

        thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();
            let mut body = Vec::new();
            loop {
                let mut content_length: Option<usize> = None;
                loop {
                    line.clear();
                    match reader.read_line(&mut line) {
                        Ok(0) | Err(_) => {
                            is_alive_reader.store(false, Ordering::Relaxed);
                            return;
                        }
                        Ok(_) => {}
                    }
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        break;
                    }
                    if trimmed.len() >= 15 && trimmed[..15].eq_ignore_ascii_case("content-length:") {
                        if let Ok(len) = trimmed[15..].trim().parse::<usize>() {
                            content_length = Some(len);
                        }
                    }
                }

                let len = match content_length {
                    Some(l) => l,
                    None => continue,
                };

                body.resize(len, 0);
                if reader.read_exact(&mut body).is_err() {
                    is_alive_reader.store(false, Ordering::Relaxed);
                    return;
                }

                if let Ok(msg) = serde_json::from_slice::<Value>(&body) {
                    if let Some(id) = msg.get("id").and_then(|v| v.as_u64()) {
                        let sender = {
                            let mut map = pending_for_reader.lock().unwrap_or_else(|e| e.into_inner());
                            map.remove(&id)
                        };
                        if let Some(tx) = sender {
                            let _ = tx.send(msg);
                        }
                    }
                }
            }
        });

        log(&format!("Successfully connected to glsl_analyzer backend at '{binary_path}'"));

        Some(Self {
            stdin: Mutex::new(stdin),
            child: Mutex::new(child),
            pending_requests,
            next_id: AtomicU64::new(1000),
            is_alive,
        })
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive.load(Ordering::Relaxed)
    }

    pub fn send_notification(&self, method: &str, params: Value) {
        if !self.is_alive() {
            return;
        }
        let notif = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params
        });
        let payload = notif.to_string();
        let wire = format!("Content-Length: {}\r\n\r\n{}", payload.len(), payload);
        if let Ok(mut stdin) = self.stdin.lock() {
            if stdin.write_all(wire.as_bytes()).is_err() || stdin.flush().is_err() {
                self.is_alive.store(false, Ordering::Relaxed);
            }
        }
    }

    pub fn send_request(&self, method: &str, params: Value, timeout: Duration) -> Option<Value> {
        if !self.is_alive() {
            return None;
        }
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let req = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });
        let payload = req.to_string();
        let wire = format!("Content-Length: {}\r\n\r\n{}", payload.len(), payload);

        let (tx, rx) = mpsc::channel();
        {
            let mut map = self.pending_requests.lock().unwrap_or_else(|e| e.into_inner());
            map.insert(id, tx);
        }

        let write_ok = if let Ok(mut stdin) = self.stdin.lock() {
            if stdin.write_all(wire.as_bytes()).is_err() || stdin.flush().is_err() {
                self.is_alive.store(false, Ordering::Relaxed);
                false
            } else {
                true
            }
        } else {
            false
        };

        if !write_ok {
            let mut map = self.pending_requests.lock().unwrap_or_else(|e| e.into_inner());
            map.remove(&id);
            return None;
        }

        let resp = match rx.recv_timeout(timeout) {
            Ok(resp) => {
                self.pending_requests.lock().unwrap_or_else(|e| e.into_inner()).remove(&id);
                resp
            }
            Err(_) => {
                self.pending_requests.lock().unwrap_or_else(|e| e.into_inner()).remove(&id);
                return None;
            }
        };

        resp.get("result").cloned()
    }
}

impl Drop for AnalyzerBridge {
    fn drop(&mut self) {
        // Send LSP shutdown + exit before marking dead
        let _ = self.send_request("shutdown", json!(null), Duration::from_millis(300));
        self.send_notification("exit", json!(null));
        self.is_alive.store(false, Ordering::Relaxed);
        // Force kill if still running and reap child to prevent zombies
        if let Ok(mut child) = self.child.lock() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_real_bridge() {
        let p = match crate::find_glsl_analyzer(None) {
            Some(p) => p,
            None => return,
        };
        let bridge = AnalyzerBridge::start(&p).expect("start analyzer bridge");
        let init_params = json!({
            "processId": std::process::id(),
            "rootUri": null,
            "capabilities": {}
        });
        let t0 = std::time::Instant::now();
        let init_res = bridge.send_request("initialize", init_params, Duration::from_secs(2));
        println!("init time: {:?}", t0.elapsed());
        println!("init res: {:?}", init_res);
        assert!(init_res.is_some());
        bridge.send_notification("initialized", json!({}));

        let did_open = json!({
            "textDocument": {
                "uri": "file:///test.frag",
                "languageId": "glsl",
                "version": 1,
                "text": "#version 460 core\nvoid main() {\n    vec4 myVec = vec4(1.0);\n    nor\n}\n"
            }
        });
        bridge.send_notification("textDocument/didOpen", did_open);

        let t1 = std::time::Instant::now();
        let comp_res = bridge.send_request("textDocument/completion", json!({
            "textDocument": { "uri": "file:///test.frag" },
            "position": { "line": 3, "character": 7 }
        }), Duration::from_secs(2));
        println!("comp time: {:?}", t1.elapsed());

        let mut doc_cache = HashMap::new();
        doc_cache.insert("file:///test.frag".to_string(), "#version 460 core\nvoid main() {\n    vec4 myVec = vec4(1.0);\n    nor\n}\n".to_string());
        let req = json!({
            "params": {
                "textDocument": { "uri": "file:///test.frag" },
                "position": { "line": 3, "character": 7 }
            }
        });

        let t2 = std::time::Instant::now();
        let enhanced = crate::enhance_analyzer_completions(&req, comp_res.unwrap(), &doc_cache);
        println!("enhance time: {:?}", t2.elapsed());
        println!("enhanced count: {}", enhanced.as_array().map(|a| a.len()).unwrap_or(0));
    }
}
