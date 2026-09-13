use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use serde_json::{json, Value};

fn get_log_path() -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push("glsl_validator.log");
    p
}

fn log(msg: &str) {
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(get_log_path())
    {
        let _ = writeln!(f, "[glsl_validator] {}", msg);
    }
}

fn find_glslang_validator() -> String {
    // 1. Explicit environment variable
    if let Ok(env_path) = std::env::var("GLSLANG_VALIDATOR_PATH") {
        if Path::new(&env_path).exists() {
            return env_path;
        }
    }

    // 2. Vulkan SDK standard location
    if let Ok(vk_sdk) = std::env::var("VULKAN_SDK") {
        let vk_bin = Path::new(&vk_sdk).join("bin").join(if cfg!(windows) {
            "glslangValidator.exe"
        } else {
            "glslangValidator"
        });
        if vk_bin.exists() {
            return vk_bin.to_string_lossy().to_string();
        }
    }

    // 3. Common Windows MSYS2 / UCRT64 path
    #[cfg(windows)]
    {
        let msys = "C:\\msys64\\ucrt64\\bin\\glslangValidator.exe";
        if Path::new(msys).exists() {
            return msys.to_string();
        }
    }

    // 4. Default to system PATH lookup
    "glslangValidator".to_string()
}

fn get_stage_from_uri(uri: &str, text: &str) -> &'static str {
    let lower = uri.to_lowercase();
    if lower.ends_with(".vert") || lower.contains(".vert.") {
        "vert"
    } else if lower.ends_with(".frag") || lower.contains(".frag.") {
        "frag"
    } else if lower.ends_with(".geom") || lower.contains(".geom.") {
        "geom"
    } else if lower.ends_with(".tesc") || lower.contains(".tesc.") {
        "tesc"
    } else if lower.ends_with(".tese") || lower.contains(".tese.") {
        "tese"
    } else if lower.ends_with(".comp") || lower.contains(".comp.") {
        "comp"
    } else if lower.ends_with(".mesh") || lower.contains(".mesh.") {
        "mesh"
    } else if lower.ends_with(".task") || lower.contains(".task.") {
        "task"
    } else if lower.ends_with(".rgen") {
        "rgen"
    } else if lower.ends_with(".rint") {
        "rint"
    } else if lower.ends_with(".rahit") {
        "rahit"
    } else if lower.ends_with(".rchit") {
        "rchit"
    } else if lower.ends_with(".rmiss") {
        "rmiss"
    } else if lower.ends_with(".rcall") {
        "rcall"
    } else {
        // Inspect content heuristics for generic .glsl/.glslh files
        if text.contains("gl_Position") {
            "vert"
        } else if text.contains("gl_FragCoord") || text.contains("gl_FragColor") {
            "frag"
        } else {
            "vert"
        }
    }
}

fn validate_shader(uri: &str, text: &str) -> Vec<Value> {
    let stage = get_stage_from_uri(uri, text);
    let compiler = find_glslang_validator();
    log(&format!("Validating uri='{uri}', stage='{stage}', compiler='{compiler}'"));

    let mut diagnostics = Vec::new();

    // Pure Desktop OpenGL validation:
    // We intentionally omit -G or -V here, because -G forces SPIR-V binary generation
    // which strictly mandates layout(location = X) on all in/out variables.
    // Pure OpenGL 4.6 GLSL allows standard `out vec3 Normal;` declarations!
    let mut child = match Command::new(&compiler)
        .args(["--stdin", "-C", "--error-column", "-S", stage])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            log(&format!("ERROR: Failed to spawn '{compiler}': {e}"));
            return diagnostics;
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(text.as_bytes());
    }

    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => {
            log(&format!("ERROR: Failed to wait on '{compiler}': {e}"));
            return diagnostics;
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let full_output = format!("{}\n{}", stdout, stderr);
    log(&format!("Compiler exit code: {:?}, output lines: {}", output.status.code(), full_output.lines().count()));

    for raw_line in full_output.lines() {
        let line = raw_line.trim();
        let is_error = line.starts_with("ERROR: ");
        let is_warning = line.starts_with("WARNING: ");

        if !is_error && !is_warning {
            continue;
        }

        let severity = if is_error { 1 } else { 2 };
        let rest = if is_error {
            &line["ERROR: ".len()..]
        } else {
            &line["WARNING: ".len()..]
        };

        // Skip compilation summary line (e.g. "2 compilation errors. No code generated.")
        if rest.contains("compilation errors") || rest.contains("No code generated") {
            continue;
        }

        log(&format!("Raw diagnostic line: {line}"));

        // Format:
        // 0:line:col: message
        // 0:line: message
        // stdin:line:col: message
        let parts: Vec<&str> = rest.splitn(4, ':').collect();
        if parts.len() < 3 {
            continue;
        }

        let line_num: u32 = match parts[1].trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let line_0 = line_num.saturating_sub(1);

        let (start_col, end_col, message) = if parts.len() == 4 {
            if let Ok(c) = parts[2].trim().parse::<u32>() {
                let col = c.saturating_sub(1);
                (col, col.saturating_add(4).max(col + 1), parts[3].trim().to_string())
            } else {
                (0, 999, format!("{}: {}", parts[2].trim(), parts[3].trim()))
            }
        } else {
            (0, 999, parts[2].trim().to_string())
        };

        let clean_msg = message.trim_start_matches("'' :").trim().to_string();

        diagnostics.push(json!({
            "range": {
                "start": { "line": line_0, "character": start_col },
                "end": { "line": line_0, "character": end_col }
            },
            "severity": severity,
            "source": "glslangValidator",
            "message": clean_msg
        }));
    }

    log(&format!("Found {} diagnostic(s) for '{uri}'", diagnostics.len()));
    diagnostics
}

fn send_lsp_message<W: Write>(writer: &mut W, msg: &Value) -> io::Result<()> {
    let body = serde_json::to_string(msg)?;
    let header = format!("Content-Length: {}\r\n\r\n", body.len());
    writer.write_all(header.as_bytes())?;
    writer.write_all(body.as_bytes())?;
    writer.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    log("=== glsl_validator started ===");
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut doc_cache: HashMap<String, String> = HashMap::new();

    loop {
        let mut content_length: Option<usize> = None;

        loop {
            let mut header_line = String::new();
            if stdin_lock.read_line(&mut header_line)? == 0 {
                log("EOF reached on stdin, exiting.");
                return Ok(());
            }

            let trimmed = header_line.trim();
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

        let mut body_buf = vec![0u8; len];
        stdin_lock.read_exact(&mut body_buf)?;

        let msg: Value = match serde_json::from_slice(&body_buf) {
            Ok(v) => v,
            Err(e) => {
                log(&format!("Failed to parse JSON body: {e}"));
                continue;
            }
        };

        let method = msg["method"].as_str().unwrap_or("");
        let id = msg.get("id");

        if let Some(req_id) = id {
            log(&format!("Handling request id={:?}, method='{method}'", req_id));
            match method {
                "initialize" => {
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": {
                            "capabilities": {
                                "textDocumentSync": 1
                            }
                        }
                    });
                    send_lsp_message(&mut stdout_lock, &resp)?;
                }
                "shutdown" => {
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": null
                    });
                    send_lsp_message(&mut stdout_lock, &resp)?;
                }
                _ => {
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": null
                    });
                    send_lsp_message(&mut stdout_lock, &resp)?;
                }
            }
            continue;
        }

        match method {
            "initialized" => {
                log("Received initialized notification.");
            }
            "exit" => {
                log("Received exit notification.");
                std::process::exit(0);
            }
            "textDocument/didOpen" => {
                if let Some(doc) = msg["params"]["textDocument"].as_object() {
                    let uri = doc.get("uri").and_then(|u| u.as_str()).unwrap_or("");
                    let text = doc.get("text").and_then(|t| t.as_str()).unwrap_or("");
                    log(&format!("didOpen: {uri} (length={})", text.len()));

                    doc_cache.insert(uri.to_string(), text.to_string());
                    let diagnostics = validate_shader(uri, text);

                    let notif = json!({
                        "jsonrpc": "2.0",
                        "method": "textDocument/publishDiagnostics",
                        "params": {
                            "uri": uri,
                            "diagnostics": diagnostics
                        }
                    });
                    send_lsp_message(&mut stdout_lock, &notif)?;
                }
            }
            "textDocument/didChange" => {
                if let Some(params) = msg["params"].as_object() {
                    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
                    if let Some(changes) = params.get("contentChanges").and_then(|c| c.as_array()) {
                        if let Some(first_change) = changes.first() {
                            if let Some(text) = first_change["text"].as_str() {
                                log(&format!("didChange: {uri} (length={})", text.len()));
                                doc_cache.insert(uri.to_string(), text.to_string());
                                let diagnostics = validate_shader(uri, text);

                                let notif = json!({
                                    "jsonrpc": "2.0",
                                    "method": "textDocument/publishDiagnostics",
                                    "params": {
                                        "uri": uri,
                                        "diagnostics": diagnostics
                                    }
                                });
                                send_lsp_message(&mut stdout_lock, &notif)?;
                            }
                        }
                    }
                }
            }
            "textDocument/didSave" => {
                if let Some(params) = msg["params"].as_object() {
                    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
                    log(&format!("didSave: {uri}"));
                    if let Some(text) = doc_cache.get(uri) {
                        let diagnostics = validate_shader(uri, text);
                        let notif = json!({
                            "jsonrpc": "2.0",
                            "method": "textDocument/publishDiagnostics",
                            "params": {
                                "uri": uri,
                                "diagnostics": diagnostics
                            }
                        });
                        send_lsp_message(&mut stdout_lock, &notif)?;
                    }
                }
            }
            "textDocument/didClose" => {
                if let Some(params) = msg["params"].as_object() {
                    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
                    log(&format!("didClose: {uri}"));
                    doc_cache.remove(uri);

                    let notif = json!({
                        "jsonrpc": "2.0",
                        "method": "textDocument/publishDiagnostics",
                        "params": {
                            "uri": uri,
                            "diagnostics": []
                        }
                    });
                    send_lsp_message(&mut stdout_lock, &notif)?;
                }
            }
            other => {
                log(&format!("Ignored notification: {other}"));
            }
        }
    }
}