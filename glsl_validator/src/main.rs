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

fn is_in_path(cmd: &str) -> bool {
    let mut check = Command::new(cmd);
    check.arg("--version");
    check.stdout(Stdio::null());
    check.stderr(Stdio::null());
    check.status().is_ok()
}

fn find_glslang_validator() -> Option<String> {
    // 1. Explicit environment variable
    if let Ok(env_path) = std::env::var("GLSLANG_VALIDATOR_PATH") {
        if Path::new(&env_path).exists() {
            return Some(env_path);
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
            return Some(vk_bin.to_string_lossy().to_string());
        }
    }

    // 3. Common Windows MSYS2 / UCRT64 path
    #[cfg(windows)]
    {
        let msys = "C:\\msys64\\ucrt64\\bin\\glslangValidator.exe";
        if Path::new(msys).exists() {
            return Some(msys.to_string());
        }
    }

    // 4. Default to system PATH lookup
    if is_in_path("glslangValidator") {
        return Some("glslangValidator".to_string());
    }
    if is_in_path("glslang") {
        return Some("glslang".to_string());
    }

    None
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetApi {
    OpenGl,
    Vulkan,
}

impl TargetApi {
    pub fn parse_target(s: &str) -> Self {
        if s.eq_ignore_ascii_case("vulkan") || s.eq_ignore_ascii_case("vk") {
            TargetApi::Vulkan
        } else {
            TargetApi::OpenGl
        }
    }

    pub fn flag(&self) -> &'static str {
        match self {
            TargetApi::OpenGl => "-C",
            TargetApi::Vulkan => "-V",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            TargetApi::OpenGl => "OpenGL 4.6",
            TargetApi::Vulkan => "Vulkan",
        }
    }
}

pub fn detect_target_api(text: &str, default_target: TargetApi) -> TargetApi {
    for line in text.lines().take(10) {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with("#pragma") {
            let lower = trimmed.to_lowercase();
            if lower.contains("@target: vulkan")
                || lower.contains("@target:vulkan")
                || lower.contains("@api: vulkan")
                || lower.contains("target(vulkan)")
            {
                return TargetApi::Vulkan;
            }
            if lower.contains("@target: opengl")
                || lower.contains("@target:opengl")
                || lower.contains("@api: opengl")
                || lower.contains("target(opengl)")
            {
                return TargetApi::OpenGl;
            }
        }
    }
    default_target
}

fn validate_shader(uri: &str, text: &str, default_target: TargetApi) -> Vec<Value> {
    let stage = get_stage_from_uri(uri, text);
    let target = detect_target_api(text, default_target);
    let mut diagnostics = Vec::new();

    let compiler = match find_glslang_validator() {
        Some(c) => c,
        None => {
            log("WARNING: glslangValidator not found on system.");
            diagnostics.push(json!({
                "range": {
                    "start": { "line": 0, "character": 0 },
                    "end": { "line": 0, "character": 999 }
                },
                "severity": 2, // Warning
                "source": "glsl_validator",
                "message": "glslangValidator not found. Please install Vulkan SDK or glslang to enable compile-time linting (see extension README)."
            }));
            return diagnostics;
        }
    };

    log(&format!(
        "Validating uri='{uri}', stage='{stage}', target='{:?}' (flag='{}'), compiler='{compiler}'",
        target,
        target.flag()
    ));

    let mut child = match Command::new(&compiler)
        .args(["--stdin", target.flag(), "--error-column", "-S", stage])
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
        let source_label = format!("glslangValidator ({})", target.display_name());

        diagnostics.push(json!({
            "range": {
                "start": { "line": line_0, "character": start_col },
                "end": { "line": line_0, "character": end_col }
            },
            "severity": severity,
            "source": source_label,
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

    let mut default_target = TargetApi::OpenGl;
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
                    if let Some(opts) = msg["params"].get("initializationOptions") {
                        if let Some(t) = opts.get("target_api").and_then(|v| v.as_str()) {
                            default_target = TargetApi::parse_target(t);
                            log(&format!("Initialized with target_api={:?}", default_target));
                        }
                    }
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
            "workspace/didChangeConfiguration" => {
                log("Received workspace/didChangeConfiguration notification.");
                if let Some(settings) = msg["params"].get("settings") {
                    let mut new_target = None;
                    if let Some(t) = settings.get("target_api").and_then(|v| v.as_str()) {
                        new_target = Some(TargetApi::parse_target(t));
                    } else if let Some(t) = settings.get("glsl_validator").and_then(|g| g.get("target_api")).and_then(|v| v.as_str()) {
                        new_target = Some(TargetApi::parse_target(t));
                    } else if let Some(t) = settings.get("initialization_options").and_then(|g| g.get("target_api")).and_then(|v| v.as_str()) {
                        new_target = Some(TargetApi::parse_target(t));
                    }
                    if let Some(nt) = new_target {
                        if nt != default_target {
                            log(&format!("Updated default_target from {:?} to {:?}", default_target, nt));
                            default_target = nt;
                            for (uri, text) in &doc_cache {
                                let diagnostics = validate_shader(uri, text, default_target);
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
            "textDocument/didOpen" => {
                if let Some(doc) = msg["params"]["textDocument"].as_object() {
                    let uri = doc.get("uri").and_then(|u| u.as_str()).unwrap_or("");
                    let text = doc.get("text").and_then(|t| t.as_str()).unwrap_or("");
                    log(&format!("didOpen: {uri} (length={})", text.len()));

                    doc_cache.insert(uri.to_string(), text.to_string());
                    let diagnostics = validate_shader(uri, text, default_target);

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
                                let diagnostics = validate_shader(uri, text, default_target);

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
                        let diagnostics = validate_shader(uri, text, default_target);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_detection_by_extension() {
        assert_eq!(get_stage_from_uri("file:///project/test.vert", ""), "vert");
        assert_eq!(get_stage_from_uri("file:///project/test.frag", ""), "frag");
        assert_eq!(get_stage_from_uri("file:///project/test.geom", ""), "geom");
        assert_eq!(get_stage_from_uri("file:///project/test.tesc", ""), "tesc");
        assert_eq!(get_stage_from_uri("file:///project/test.tese", ""), "tese");
        assert_eq!(get_stage_from_uri("file:///project/test.comp", ""), "comp");
        assert_eq!(get_stage_from_uri("file:///project/test.mesh", ""), "mesh");
        assert_eq!(get_stage_from_uri("file:///project/test.task", ""), "task");
        assert_eq!(get_stage_from_uri("file:///project/test.rgen", ""), "rgen");
    }

    #[test]
    fn test_stage_detection_heuristics() {
        assert_eq!(get_stage_from_uri("file:///project/shader.glsl", "void main() { gl_Position = vec4(1.0); }"), "vert");
        assert_eq!(get_stage_from_uri("file:///project/shader.glsl", "void main() { gl_FragCoord.xy; }"), "frag");
        assert_eq!(get_stage_from_uri("file:///project/shader.glslh", "// header file"), "vert");
    }

    #[test]
    fn test_target_api_parsing() {
        assert_eq!(TargetApi::parse_target("vulkan"), TargetApi::Vulkan);
        assert_eq!(TargetApi::parse_target("Vulkan"), TargetApi::Vulkan);
        assert_eq!(TargetApi::parse_target("vk"), TargetApi::Vulkan);
        assert_eq!(TargetApi::parse_target("opengl"), TargetApi::OpenGl);
        assert_eq!(TargetApi::parse_target("OpenGL"), TargetApi::OpenGl);
        assert_eq!(TargetApi::parse_target("gl"), TargetApi::OpenGl);
        assert_eq!(TargetApi::parse_target("anything_else"), TargetApi::OpenGl);
    }

    #[test]
    fn test_detect_target_api_directives() {
        assert_eq!(
            detect_target_api("// @target: vulkan\nvoid main() {}", TargetApi::OpenGl),
            TargetApi::Vulkan
        );
        assert_eq!(
            detect_target_api("/* @target: vulkan */\nvoid main() {}", TargetApi::OpenGl),
            TargetApi::Vulkan
        );
        assert_eq!(
            detect_target_api("#pragma target(vulkan)\nvoid main() {}", TargetApi::OpenGl),
            TargetApi::Vulkan
        );
        assert_eq!(
            detect_target_api("// standard opengl shader\nvoid main() {}", TargetApi::OpenGl),
            TargetApi::OpenGl
        );
        assert_eq!(
            detect_target_api("// standard vulkan shader\nvoid main() {}", TargetApi::Vulkan),
            TargetApi::Vulkan
        );
    }
}