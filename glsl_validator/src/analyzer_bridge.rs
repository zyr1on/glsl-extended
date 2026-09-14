// glsl_validator - analyzer_bridge.rs
// Manages glsl_analyzer child process via stdio JSON-RPC, multiplexing with glslangValidator.

use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{ChildStdin, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::log;

pub struct AnalyzerBridge {
    stdin: Mutex<ChildStdin>,
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
            loop {
                let mut content_length: Option<usize> = None;
                loop {
                    let mut line = String::new();
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
                    let lower = trimmed.to_lowercase();
                    if let Some(val) = lower.strip_prefix("content-length:") {
                        if let Ok(len) = val.trim().parse::<usize>() {
                            content_length = Some(len);
                        }
                    }
                }

                let len = match content_length {
                    Some(l) => l,
                    None => continue,
                };

                let mut body = vec![0u8; len];
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
            let _ = stdin.write_all(wire.as_bytes());
            let _ = stdin.flush();
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

        {
            let mut stdin = self.stdin.lock().ok()?;
            stdin.write_all(wire.as_bytes()).ok()?;
            stdin.flush().ok()?;
        }

        let resp = rx.recv_timeout(timeout).ok()?;
        resp.get("result").cloned()
    }
}
