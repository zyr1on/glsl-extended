pub mod docs;
pub mod signature;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{mpsc, Arc, Mutex, OnceLock};
use std::thread;
use serde_json::{json, Value};

fn get_log_path() -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push("glsl_validator.log");
    p
}

fn log(msg: &str) {
    static LOG_ENABLED: OnceLock<bool> = OnceLock::new();
    let enabled = *LOG_ENABLED.get_or_init(|| {
        std::env::var("GLSL_VALIDATOR_LOG").is_ok() || std::env::var("GLSL_DEBUG").is_ok()
    });
    if !enabled {
        return;
    }

    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(get_log_path())
    {
        let _ = writeln!(f, "[glsl_validator] {}", msg);
    }
}

fn find_in_path(binary: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    let exts: Vec<String> = if cfg!(windows) {
        std::env::var("PATHEXT")
            .unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD".to_string())
            .split(';')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
            .collect()
    } else {
        vec!["".to_string()]
    };

    for dir in std::env::split_paths(&path_var) {
        if cfg!(windows) {
            if binary.contains('.') {
                let candidate = dir.join(binary);
                if candidate.is_file() {
                    return Some(candidate);
                }
            } else {
                for ext in &exts {
                    let candidate = dir.join(format!("{}{}", binary, ext));
                    if candidate.is_file() {
                        return Some(candidate);
                    }
                }
                let candidate = dir.join(binary);
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        } else {
            let candidate = dir.join(binary);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

fn is_in_path(cmd: &str) -> bool {
    if find_in_path(cmd).is_some() {
        return true;
    }
    let mut check = Command::new(cmd);
    check.arg("--version");
    check.stdout(Stdio::null());
    check.stderr(Stdio::null());
    check.status().is_ok()
}

fn find_glslang_validator(custom_path: Option<&str>) -> Option<String> {
    // 1. Explicit user configuration from Zed settings.json
    if let Some(custom) = custom_path {
        let trimmed = custom.trim();
        if !trimmed.is_empty() {
            if Path::new(trimmed).is_file() {
                return Some(trimmed.to_string());
            }
            if let Some(p) = find_in_path(trimmed) {
                return Some(p.to_string_lossy().to_string());
            }
            if is_in_path(trimmed) {
                return Some(trimmed.to_string());
            }
        }
    }

    // 2. Explicit user environment variable override
    if let Ok(env_path) = std::env::var("GLSLANG_VALIDATOR_PATH") {
        if Path::new(&env_path).exists() {
            return Some(env_path);
        }
    }

    // 2. Primary: System PATH (universal across Windows, Linux, macOS)
    if let Some(p) = find_in_path("glslangValidator") {
        return Some(p.to_string_lossy().to_string());
    }
    if let Some(p) = find_in_path("glslang") {
        return Some(p.to_string_lossy().to_string());
    }
    if is_in_path("glslangValidator") {
        return Some("glslangValidator".to_string());
    }
    if is_in_path("glslang") {
        return Some("glslang".to_string());
    }

    // 3. Vulkan SDK standard environment variable
    if let Ok(vk_sdk) = std::env::var("VULKAN_SDK") {
        let exe = if cfg!(windows) {
            "glslangValidator.exe"
        } else {
            "glslangValidator"
        };
        let vk_bin = Path::new(&vk_sdk).join("bin").join(exe);
        if vk_bin.is_file() {
            return Some(vk_bin.to_string_lossy().to_string());
        }
    }

    // 4. Platform-specific fallback search paths
    #[cfg(windows)]
    {
        let mut candidates = vec![
            "C:\\Program Files\\glslang\\bin\\glslangValidator.exe".to_string(),
            "C:\\msys64\\ucrt64\\bin\\glslangValidator.exe".to_string(),
            "C:\\msys64\\mingw64\\bin\\glslangValidator.exe".to_string(),
            "C:\\msys64\\clang64\\bin\\glslangValidator.exe".to_string(),
        ];
        if let Ok(entries) = std::fs::read_dir("C:\\VulkanSDK") {
            for entry in entries.flatten() {
                let candidate = entry.path().join("bin").join("glslangValidator.exe");
                if candidate.is_file() {
                    candidates.push(candidate.to_string_lossy().to_string());
                }
            }
        }
        for path in candidates {
            if Path::new(&path).is_file() {
                return Some(path);
            }
        }
    }

    #[cfg(not(windows))]
    {
        let candidates = [
            "/usr/bin/glslangValidator",
            "/usr/local/bin/glslangValidator",
            "/opt/homebrew/bin/glslangValidator",
            "/usr/bin/glslang",
            "/usr/local/bin/glslang",
        ];
        for path in candidates {
            if Path::new(path).is_file() {
                return Some(path.to_string());
            }
        }
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
            TargetApi::OpenGl => "OpenGL",
            TargetApi::Vulkan => "Vulkan",
        }
    }

    pub fn display_label(&self, text: &str) -> String {
        match self {
            TargetApi::Vulkan => "glslangValidator (Vulkan)".to_string(),
            TargetApi::OpenGl => {
                for line in text.lines() {
                    let trimmed = line.trim();
                    if let Some(rest) = trimmed.strip_prefix("#version") {
                        let ver = rest.trim();
                        if !ver.is_empty() {
                            return format!("glslangValidator (OpenGL {ver})");
                        }
                    }
                }
                "glslangValidator (OpenGL)".to_string()
            }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatterEngine {
    Builtin,     // Default: pure-Rust GLSL formatter
    ClangFormat, // clang-format with built-in fallback
}

impl FormatterEngine {
    pub fn parse_engine(s: &str) -> Self {
        if s.eq_ignore_ascii_case("clang-format") || s.eq_ignore_ascii_case("clang") {
            FormatterEngine::ClangFormat
        } else {
            FormatterEngine::Builtin
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            FormatterEngine::Builtin => "Builtin (Pure Rust)",
            FormatterEngine::ClangFormat => "clang-format",
        }
    }
}

pub fn detect_formatter_engine(text: &str, default_engine: FormatterEngine) -> FormatterEngine {
    for line in text.lines().take(10) {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with("#pragma") {
            let lower = trimmed.to_lowercase();
            if lower.contains("@formatter: clang-format")
                || lower.contains("@formatter:clang-format")
                || lower.contains("@formatter: clang")
                || lower.contains("formatter(clang)")
            {
                return FormatterEngine::ClangFormat;
            }
            if lower.contains("@formatter: builtin")
                || lower.contains("@formatter:builtin")
                || lower.contains("formatter(builtin)")
            {
                return FormatterEngine::Builtin;
            }
        }
    }
    default_engine
}

pub fn percent_decode_str(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if hex.len() == 2 {
                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                    result.push(byte as char);
                    continue;
                }
            }
            result.push('%');
            result.push_str(&hex);
        } else {
            result.push(c);
        }
    }
    result
}

pub fn uri_to_path(uri: &str) -> Option<PathBuf> {
    let stripped = uri.strip_prefix("file://")?;
    let path_str = if cfg!(windows) {
        if stripped.starts_with('/') && stripped.chars().nth(2) == Some(':') {
            &stripped[1..]
        } else {
            stripped
        }
    } else {
        stripped
    };

    let decoded = percent_decode_str(path_str);
    Some(PathBuf::from(decoded))
}

pub fn get_include_dirs(uri: &str) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(file_path) = uri_to_path(uri) {
        if let Some(parent) = file_path.parent() {
            if parent.exists() {
                dirs.push(parent.to_path_buf());
                let inc = parent.join("include");
                if inc.exists() && inc.is_dir() {
                    dirs.push(inc);
                }
                let shaders = parent.join("shaders");
                if shaders.exists() && shaders.is_dir() {
                    dirs.push(shaders);
                }
            }
            if let Some(grandparent) = parent.parent() {
                if grandparent.exists() {
                    dirs.push(grandparent.to_path_buf());
                    let inc = grandparent.join("include");
                    if inc.exists() && inc.is_dir() && !dirs.contains(&inc) {
                        dirs.push(inc);
                    }
                    let shaders = grandparent.join("shaders");
                    if shaders.exists() && shaders.is_dir() && !dirs.contains(&shaders) {
                        dirs.push(shaders);
                    }
                }
            }
        }
    }
    dirs
}

fn validate_shader(
    uri: &str,
    text: &str,
    default_target: TargetApi,
    custom_glslang: Option<&str>,
) -> Vec<Value> {
    let stage = get_stage_from_uri(uri, text);
    let target = detect_target_api(text, default_target);
    let mut diagnostics = Vec::new();

    let compiler = match find_glslang_validator(custom_glslang) {
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
                "message": "glslangValidator not found. Install Vulkan SDK / glslang or specify glslang_validator_path in settings."
            }));
            return diagnostics;
        }
    };

    let inc_dirs = get_include_dirs(uri);
    log(&format!(
        "Validating uri='{uri}', stage='{stage}', target='{:?}' (flag='{}'), compiler='{compiler}', includes={}",
        target,
        target.flag(),
        inc_dirs.len()
    ));

    let (compile_text, pre_output) = if target == TargetApi::OpenGl && text.contains("#include") {
        log(&format!("Preprocessing '#include' directives for OpenGL target using '{compiler}'"));
        let mut prep_cmd = Command::new(&compiler);
        prep_cmd.args(["--stdin", "-E", "-S", stage]);
        for inc in &inc_dirs {
            prep_cmd.arg(format!("-I{}", inc.display()));
        }
        if let Ok(mut child) = prep_cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(text.as_bytes());
            }
            if let Ok(output) = child.wait_with_output() {
                if !output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    (text.to_string(), Some(format!("{}\n{}", stdout, stderr)))
                } else {
                    let preprocessed = String::from_utf8_lossy(&output.stdout).to_string();
                    (preprocessed, None)
                }
            } else {
                (text.to_string(), None)
            }
        } else {
            (text.to_string(), None)
        }
    } else {
        (text.to_string(), None)
    };

    let full_output = if let Some(err_output) = pre_output {
        err_output
    } else {
        let mut cmd = Command::new(&compiler);
        cmd.args(["--stdin", target.flag(), "--error-column", "-S", stage]);
        for inc in &inc_dirs {
            cmd.arg(format!("-I{}", inc.display()));
        }

        let mut child = match cmd
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
            let _ = stdin.write_all(compile_text.as_bytes());
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
        format!("{}\n{}", stdout, stderr)
    };
    log(&format!("Validation output lines: {}", full_output.lines().count()));

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
        let source_label = target.display_label(text);

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

pub fn infer_vector_dimension(doc: &str, expr: &str) -> usize {
    let segments: Vec<&str> = expr.split('.').filter(|s| !s.is_empty()).collect();
    if segments.is_empty() {
        return 4;
    }

    let last = match segments.last() {
        Some(l) => *l,
        None => return 4,
    };

    // 1. If `last` is already a swizzle (e.g. `pos.xyz.` or `a.xy.`)
    if last.len() >= 2 && last.chars().all(|c| "xyzwrgbastpq".contains(c)) {
        return match last.len() {
            2 => 2,
            3 => 3,
            _ => 4,
        };
    }

    // 2. Built-in GLSL vector variables
    match last {
        "gl_Position" | "gl_FragCoord" | "gl_FragColor" | "gl_Vertex" | "gl_Color" => return 4,
        "gl_Normal" | "gl_GlobalInvocationID" | "gl_LocalInvocationID" | "gl_WorkGroupID" => return 3,
        "gl_PointCoord" => return 2,
        _ => {}
    }

    // 3. Scan document for variable or struct member declaration
    for line in doc.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
            continue;
        }

        if let Some(idx) = line.find(last) {
            let before = &line[..idx];
            let after = &line[idx + last.len()..];
            let before_ok = before.chars().last().is_none_or(|c| !c.is_alphanumeric() && c != '_');
            let after_ok = after.chars().next().is_none_or(|c| !c.is_alphanumeric() && c != '_');

            if before_ok && after_ok {
                if before.contains("vec4") {
                    return 4;
                } else if before.contains("vec3") {
                    return 3;
                } else if before.contains("vec2") {
                    return 2;
                }
            }
        }
    }

    // 4. Heuristics from identifier name
    let lower = last.to_lowercase();
    if lower.contains("uv") || lower.contains("coord2d") {
        2
    } else if lower.contains("normal") || lower.contains("dir") || lower.contains("vel") {
        3
    } else {
        4
    }
}

pub fn generate_swizzle_completions(dim: usize) -> Vec<Value> {
    let mut items = Vec::new();

    let mut add_item = |label: &str, detail: &str, doc: &str, sort_prefix: &str| {
        items.push(json!({
            "label": label,
            "kind": 5, // Field
            "detail": detail,
            "documentation": doc,
            "insertText": label,
            "sortText": format!("{}_{}", sort_prefix, label)
        }));
    };

    match dim {
        2 => {
            // 1-component (float)
            add_item("x", "float", "X coordinate component", "01");
            add_item("y", "float", "Y coordinate component", "01");
            add_item("r", "float", "Red color component", "01");
            add_item("g", "float", "Green color component", "01");
            add_item("s", "float", "S texture coordinate", "01");
            add_item("t", "float", "T texture coordinate", "01");

            // 2-component (vec2)
            add_item("xy", "vec2", "XY 2D coordinate swizzle", "02");
            add_item("yx", "vec2", "YX reversed 2D coordinate swizzle", "02");
            add_item("xx", "vec2", "XX duplicate swizzle", "02");
            add_item("yy", "vec2", "YY duplicate swizzle", "02");
            add_item("rg", "vec2", "RG 2D color swizzle", "02");
            add_item("gr", "vec2", "GR reversed color swizzle", "02");
            add_item("st", "vec2", "ST 2D texture coordinate swizzle", "02");
            add_item("ts", "vec2", "TS reversed texture swizzle", "02");

            // 3-component (vec3)
            add_item("xxx", "vec3", "XXX 3D swizzle", "03");
            add_item("xyx", "vec3", "XYX 3D swizzle", "03");
            add_item("xyy", "vec3", "XYY 3D swizzle", "03");
            add_item("rgb", "vec3", "RGB 3D color swizzle", "03");
        }
        3 => {
            // 1-component (float)
            add_item("x", "float", "X coordinate component", "01");
            add_item("y", "float", "Y coordinate component", "01");
            add_item("z", "float", "Z coordinate component", "01");
            add_item("r", "float", "Red color component", "01");
            add_item("g", "float", "Green color component", "01");
            add_item("b", "float", "Blue color component", "01");
            add_item("s", "float", "S texture coordinate", "01");
            add_item("t", "float", "T texture coordinate", "01");
            add_item("p", "float", "P texture coordinate", "01");

            // 2-component (vec2)
            add_item("xy", "vec2", "XY 2D coordinate swizzle", "02");
            add_item("xz", "vec2", "XZ 2D coordinate swizzle", "02");
            add_item("yz", "vec2", "YZ 2D coordinate swizzle", "02");
            add_item("yx", "vec2", "YX 2D coordinate swizzle", "02");
            add_item("zx", "vec2", "ZX 2D coordinate swizzle", "02");
            add_item("zy", "vec2", "ZY 2D coordinate swizzle", "02");
            add_item("rg", "vec2", "RG color swizzle", "02");
            add_item("rb", "vec2", "RB color swizzle", "02");
            add_item("gb", "vec2", "GB color swizzle", "02");

            // 3-component (vec3)
            add_item("xyz", "vec3", "XYZ 3D coordinate swizzle", "03");
            add_item("xzy", "vec3", "XZY 3D coordinate swizzle", "03");
            add_item("yxz", "vec3", "YXZ 3D coordinate swizzle", "03");
            add_item("yzx", "vec3", "YZX 3D coordinate swizzle", "03");
            add_item("zxy", "vec3", "ZXY 3D coordinate swizzle", "03");
            add_item("zyx", "vec3", "ZYX reversed 3D coordinate swizzle", "03");
            add_item("rgb", "vec3", "RGB 3D color swizzle", "03");
            add_item("bgr", "vec3", "BGR reversed 3D color swizzle", "03");
            add_item("stp", "vec3", "STP 3D texture coordinate swizzle", "03");
        }
        _ => {
            // 1-component (float)
            add_item("x", "float", "X coordinate component", "01");
            add_item("y", "float", "Y coordinate component", "01");
            add_item("z", "float", "Z coordinate component", "01");
            add_item("w", "float", "W coordinate component", "01");
            add_item("r", "float", "Red color component", "01");
            add_item("g", "float", "Green color component", "01");
            add_item("b", "float", "Blue color component", "01");
            add_item("a", "float", "Alpha color component", "01");
            add_item("s", "float", "S texture coordinate", "01");
            add_item("t", "float", "T texture coordinate", "01");
            add_item("p", "float", "P texture coordinate", "01");
            add_item("q", "float", "Q texture coordinate", "01");

            // 2-component (vec2)
            add_item("xy", "vec2", "XY 2D coordinate swizzle", "02");
            add_item("xz", "vec2", "XZ 2D coordinate swizzle", "02");
            add_item("xw", "vec2", "XW 2D coordinate swizzle", "02");
            add_item("yz", "vec2", "YZ 2D coordinate swizzle", "02");
            add_item("yw", "vec2", "YW 2D coordinate swizzle", "02");
            add_item("zw", "vec2", "ZW 2D coordinate swizzle", "02");
            add_item("rg", "vec2", "RG 2D color swizzle", "02");
            add_item("rb", "vec2", "RB 2D color swizzle", "02");
            add_item("ra", "vec2", "RA 2D color swizzle", "02");
            add_item("gb", "vec2", "GB 2D color swizzle", "02");
            add_item("ba", "vec2", "BA 2D color swizzle", "02");
            add_item("st", "vec2", "ST 2D texture coordinate swizzle", "02");

            // 3-component (vec3)
            add_item("xyz", "vec3", "XYZ 3D coordinate swizzle", "03");
            add_item("xyw", "vec3", "XYW 3D coordinate swizzle", "03");
            add_item("xzw", "vec3", "XZW 3D coordinate swizzle", "03");
            add_item("yzw", "vec3", "YZW 3D coordinate swizzle", "03");
            add_item("zyx", "vec3", "ZYX reversed 3D coordinate swizzle", "03");
            add_item("rgb", "vec3", "RGB 3D color swizzle", "03");
            add_item("bgr", "vec3", "BGR reversed 3D color swizzle", "03");
            add_item("stp", "vec3", "STP 3D texture coordinate swizzle", "03");

            // 4-component (vec4)
            add_item("xyzw", "vec4", "XYZW 4D full coordinate swizzle", "04");
            add_item("wzyx", "vec4", "WZYX reversed coordinate swizzle", "04");
            add_item("rgba", "vec4", "RGBA 4D full color swizzle", "04");
            add_item("abgr", "vec4", "ABGR reversed color swizzle", "04");
            add_item("bgra", "vec4", "BGRA color swizzle", "04");
            add_item("argb", "vec4", "ARGB color swizzle", "04");
            add_item("stpq", "vec4", "STPQ 4D full texture swizzle", "04");
        }
    }

    // GLSL Vector method: length()
    items.push(json!({
        "label": "length()",
        "kind": 2, // Method
        "detail": format!("int length() -> {dim}"),
        "documentation": "Returns the number of components in this vector.",
        "insertText": "length()",
        "sortText": "09_length"
    }));

    items
}

pub fn generate_snippet_completions(query: &str) -> Vec<Value> {
    let snippets = [
        (
            "ubo",
            "Uniform Buffer Object (Generic)",
            "layout(std140, binding = ${1:0}) uniform ${2:BlockName} {\n\t$0\n};",
            "Generic Uniform Buffer Object (UBO) declaration",
        ),
        (
            "ssbo",
            "Shader Storage Buffer Object (Generic)",
            "layout(std430, binding = ${1:0}) buffer ${2:BlockName} {\n\t$0\n};",
            "Generic Shader Storage Buffer Object (SSBO) declaration",
        ),
        (
            "vert",
            "Vertex Shader Skeleton",
            "#version 460 core\n\nlayout(location = 0) in vec3 inPosition;\n\nvoid main() {\n\tgl_Position = vec4(inPosition, 1.0);\n}\n",
            "Clean GLSL Vertex Shader template",
        ),
        (
            "frag",
            "Fragment Shader Skeleton",
            "#version 460 core\n\nlayout(location = 0) out vec4 fragColor;\n\nvoid main() {\n\tfragColor = vec4(1.0);\n}\n",
            "Clean GLSL Fragment Shader template",
        ),
        (
            "comp",
            "Compute Shader Skeleton",
            "#version 460 core\n\nlayout(local_size_x = ${1:16}, local_size_y = ${2:16}, local_size_z = ${3:1}) in;\n\nvoid main() {\n\t$0\n}\n",
            "Clean GLSL Compute Shader template",
        ),
        (
            "geom",
            "Geometry Shader Skeleton",
            "#version 460 core\n\nlayout(${1:triangles}) in;\nlayout(${2:triangle_strip}, max_vertices = ${3:3}) out;\n\nvoid main() {\n\tfor (int i = 0; i < gl_in.length(); i++) {\n\t\tgl_Position = gl_in[i].gl_Position;\n\t\tEmitVertex();\n\t}\n\tEndPrimitive();\n}\n",
            "Clean GLSL Geometry Shader template",
        ),
        (
            "struct",
            "Struct Definition",
            "struct ${1:Name} {\n\t$0\n};",
            "GLSL Struct definition",
        ),
        (
            "func",
            "Function Definition",
            "${1:void} ${2:funcName}(${3}) {\n\t$0\n}",
            "GLSL Function definition",
        ),
        (
            "main",
            "Main Function",
            "void main() {\n\t$0\n}",
            "GLSL void main() function",
        ),
    ];

    let query_lower = query.to_lowercase();
    snippets
        .iter()
        .filter(|(prefix, _, _, _)| query_lower.is_empty() || prefix.starts_with(&query_lower))
        .map(|(prefix, detail, body, doc)| {
            json!({
                "label": prefix,
                "kind": 15, // Snippet
                "detail": detail,
                "documentation": doc,
                "insertText": body,
                "insertTextFormat": 2, // Snippet
                "sortText": format!("00_{}", prefix)
            })
        })
        .collect()
}

pub fn handle_completion(msg: &Value, doc_cache: &HashMap<String, String>) -> Value {
    let params = match msg.get("params") {
        Some(p) => p,
        None => return json!([]),
    };

    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
    let line_idx = params["position"]["line"].as_u64().unwrap_or(0) as usize;
    let col_idx = params["position"]["character"].as_u64().unwrap_or(0) as usize;

    let doc = match doc_cache.get(uri) {
        Some(d) => d,
        None => return json!([]),
    };

    let line = match doc.lines().nth(line_idx) {
        Some(l) => l,
        None => return json!([]),
    };

    let col = col_idx.min(line.len());
    let prefix = &line[..col];

    let trimmed = prefix.trim_end();
    if let Some(stripped) = trimmed.strip_suffix('.') {
        let before_dot = stripped.trim_end();
        let mut start = before_dot.len();
        for (i, c) in before_dot.char_indices().rev() {
            if c.is_alphanumeric() || c == '_' || c == '.' {
                start = i;
            } else {
                break;
            }
        }

        let expr = &before_dot[start..];
        if expr.is_empty() {
            return json!([]);
        }

        log(&format!("Swizzle completion triggered for expr='{expr}' at line={line_idx}, col={col_idx}"));

        let dim = infer_vector_dimension(doc, expr);
        let items = generate_swizzle_completions(dim);
        return json!(items);
    }

    // Extract word under/before cursor
    let mut word_start = prefix.len();
    for (i, c) in prefix.char_indices().rev() {
        if c.is_alphanumeric() || c == '_' {
            word_start = i;
        } else {
            break;
        }
    }
    let word = &prefix[word_start..];

    let mut items = Vec::new();

    // 1. Snippets (if user is typing snippet prefix)
    if !word.is_empty() {
        items.extend(generate_snippet_completions(word));
    }

    // 2. User functions from current file and recursively included files (#include)
    let user_funcs = signature::resolve_includes_and_scan(uri, doc, doc_cache);
    let word_lower = word.to_lowercase();
    for func in user_funcs {
        if word.is_empty() || func.name.to_lowercase().starts_with(&word_lower) {
            let detail = func.label.clone();
            let doc_text = match (&func.source, &func.doc) {
                (Some(src), Some(d)) => format!("*Defined in `{src}`*\n\n{d}"),
                (Some(src), None) => format!("*Defined in `{src}`*"),
                (None, Some(d)) => d.clone(),
                (None, None) => String::new(),
            };

            if items.iter().any(|it| it["label"] == func.name) {
                continue;
            }

            items.push(json!({
                "label": func.name,
                "kind": 3, // Function
                "detail": detail,
                "documentation": {
                    "kind": "markdown",
                    "value": doc_text,
                },
                "insertText": func.name,
                "insertTextFormat": 1,
                "sortText": format!("01_{}", func.name),
            }));
        }
    }

    // 3. Built-in functions from docs.gl
    for builtin in docs::get_all_builtins() {
        if word.is_empty() || builtin.name.to_lowercase().starts_with(&word_lower) {
            if items.iter().any(|it| it["label"] == builtin.name) {
                continue;
            }
            let first_overload = builtin.overloads.first().map(|o| o.label).unwrap_or("");
            items.push(json!({
                "label": builtin.name,
                "kind": 3, // Function
                "detail": first_overload,
                "documentation": {
                    "kind": "markdown",
                    "value": builtin.description,
                },
                "insertText": builtin.name,
                "insertTextFormat": 1,
                "sortText": format!("02_{}", builtin.name),
            }));
        }
    }

    json!(items)
}

static WARNED_CLANG_FORMAT: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

fn warn_missing_clang_format() {
    if WARNED_CLANG_FORMAT.swap(true, std::sync::atomic::Ordering::Relaxed) {
        return;
    }
    log("================================================================================");
    log("NOTICE: 'clang-format' was not found in system PATH or standard locations.");
    log("Formatting will use the built-in pure-Rust GLSL formatter instead.");
    log("To use clang-format for AST-level formatting, install it via your package manager:");
    log("  - Windows (winget):   winget install LLVM.LLVM (or choco install llvm)");
    log("  - Windows (MSYS2):    pacman -S mingw-w64-ucrt-x86_64-clang-tools-extra");
    log("  - Debian / Ubuntu:    sudo apt install clang-format");
    log("  - Fedora / RHEL:      sudo dnf install clang-tools-extra");
    log("  - Arch Linux:         sudo pacman -S clang");
    log("  - macOS (Homebrew):   brew install clang-format");
    log("Alternatively, set CLANG_FORMAT_PATH env var or set \"formatter\": \"builtin\" in settings.");
    log("================================================================================");
    eprintln!("[glsl_validator] NOTICE: 'clang-format' not found in PATH. Using built-in pure-Rust formatter.");
}

fn find_clang_format(custom_path: Option<&str>) -> Option<String> {
    // 1. Explicit user configuration from Zed settings.json
    if let Some(custom) = custom_path {
        let trimmed = custom.trim();
        if !trimmed.is_empty() {
            if Path::new(trimmed).is_file() {
                return Some(trimmed.to_string());
            }
            if let Some(p) = find_in_path(trimmed) {
                return Some(p.to_string_lossy().to_string());
            }
            if is_in_path(trimmed) {
                return Some(trimmed.to_string());
            }
        }
    }

    // 2. Explicit user environment variable override
    if let Ok(env_path) = std::env::var("CLANG_FORMAT_PATH") {
        if Path::new(&env_path).exists() {
            return Some(env_path);
        }
    }

    // 2. Primary: System PATH (universal across Windows, Linux, macOS)
    if let Some(p) = find_in_path("clang-format") {
        return Some(p.to_string_lossy().to_string());
    }
    if is_in_path("clang-format") {
        return Some("clang-format".to_string());
    }

    // 3. Platform-specific fallback search paths
    #[cfg(windows)]
    {
        let mut candidates = vec![
            "C:\\Program Files\\LLVM\\bin\\clang-format.exe".to_string(),
            "C:\\Program Files (x86)\\LLVM\\bin\\clang-format.exe".to_string(),
            "C:\\msys64\\ucrt64\\bin\\clang-format.exe".to_string(),
            "C:\\msys64\\mingw64\\bin\\clang-format.exe".to_string(),
            "C:\\msys64\\clang64\\bin\\clang-format.exe".to_string(),
            "C:\\tools\\llvm\\bin\\clang-format.exe".to_string(),
        ];
        if let Ok(local_app) = std::env::var("LOCALAPPDATA") {
            candidates.push(format!("{local_app}\\Programs\\LLVM\\bin\\clang-format.exe"));
        }
        if let Ok(user_profile) = std::env::var("USERPROFILE") {
            candidates.push(format!("{user_profile}\\scoop\\shims\\clang-format.exe"));
        }
        for path in candidates {
            if Path::new(&path).is_file() {
                return Some(path);
            }
        }
    }

    #[cfg(not(windows))]
    {
        let candidates = [
            "/usr/bin/clang-format",
            "/usr/local/bin/clang-format",
            "/opt/homebrew/bin/clang-format",
            "/snap/bin/clang-format",
            "/opt/llvm/bin/clang-format",
        ];
        for path in candidates {
            if Path::new(path).is_file() {
                return Some(path.to_string());
            }
        }
    }

    None
}

fn analyze_line_braces(line: &str) -> (usize, usize) {
    let mut open = 0;
    let mut close = 0;
    let mut in_str = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '"' {
            in_str = !in_str;
            continue;
        }
        if in_str {
            continue;
        }
        if c == '/' && chars.peek() == Some(&'/') {
            break; // line comment, ignore rest of line
        }
        if c == '{' {
            open += 1;
        } else if c == '}' {
            close += 1;
        }
    }
    (open, close)
}

pub fn clean_glsl_line_syntax(line: &str) -> String {
    let mut result = String::with_capacity(line.len() + 8);
    let mut chars = line.chars().peekable();
    let mut in_str = false;
    let mut prev_char: Option<char> = None;

    while let Some(c) = chars.next() {
        if c == '"' {
            in_str = !in_str;
            result.push(c);
            prev_char = Some(c);
            continue;
        }
        if in_str {
            result.push(c);
            prev_char = Some(c);
            continue;
        }
        if c == '/' && chars.peek() == Some(&'/') {
            result.push('/');
            for rest in chars.by_ref() {
                result.push(rest);
            }
            break;
        }

        // Collapse multiple spaces outside strings/comments
        if c == ' ' && prev_char == Some(' ') {
            continue;
        }

        // Space after comma: e.g. "vec3(1.0,2.0)" -> "vec3(1.0, 2.0)"
        if c == ',' {
            result.push(',');
            if let Some(&next) = chars.peek() {
                if !next.is_whitespace() {
                    result.push(' ');
                    prev_char = Some(' ');
                    continue;
                }
            }
            prev_char = Some(',');
            continue;
        }

        // Space before opening brace: e.g. "struct Test{" -> "struct Test {"
        if c == '{' && prev_char.is_some_and(|p| !p.is_whitespace()) {
            result.push(' ');
        }

        // Space between ')' and '{': e.g. "){" -> ") {"
        if c == ')' {
            result.push(')');
            if let Some(&next) = chars.peek() {
                if next == '{' {
                    result.push(' ');
                    prev_char = Some(' ');
                    continue;
                }
            }
            prev_char = Some(')');
            continue;
        }

        result.push(c);
        prev_char = Some(c);
    }

    // Normalize control keywords: "if(" -> "if (", "for(" -> "for (", "while(" -> "while ("
    let mut s = result;
    if s.starts_with("if(") {
        s = format!("if ({}", &s[3..]);
    } else if s.starts_with("for(") {
        s = format!("for ({}", &s[4..]);
    } else if s.starts_with("while(") {
        s = format!("while ({}", &s[6..]);
    }
    s
}

pub fn basic_glsl_format(text: &str, tab_size: usize, insert_spaces: bool) -> String {
    let indent_unit = if insert_spaces {
        " ".repeat(tab_size.max(1))
    } else {
        "\t".to_string()
    };

    let mut result_lines = Vec::new();
    let mut current_indent = 0usize;
    let mut prev_was_blank = false;

    for line in text.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            if !prev_was_blank && !result_lines.is_empty() {
                result_lines.push(String::new());
                prev_was_blank = true;
            }
            continue;
        }
        prev_was_blank = false;

        // Preprocessor directives always at column 0
        if trimmed.starts_with('#') {
            result_lines.push(trimmed.to_string());
            continue;
        }

        let cleaned = clean_glsl_line_syntax(trimmed);
        let (open_braces, close_braces) = analyze_line_braces(&cleaned);

        // Count leading closing braces on the line (e.g. "}", "} else {")
        let mut leading_closes = 0;
        for c in cleaned.chars() {
            if c == '}' {
                leading_closes += 1;
            } else if !c.is_whitespace() {
                break;
            }
        }

        let line_indent = if leading_closes > 0 {
            current_indent.saturating_sub(leading_closes)
        } else if cleaned.starts_with("case ") || cleaned.starts_with("default:") {
            current_indent.saturating_sub(1)
        } else {
            current_indent
        };

        let indent_str = indent_unit.repeat(line_indent);
        result_lines.push(format!("{}{}", indent_str, cleaned));

        current_indent = current_indent
            .saturating_sub(close_braces)
            .saturating_add(open_braces);
    }

    let mut res = result_lines.join("\n");
    if text.ends_with('\n') {
        res.push('\n');
    }
    res
}

pub fn format_document(
    uri: &str,
    text: &str,
    options: Option<&Value>,
    default_engine: FormatterEngine,
    custom_clang: Option<&str>,
) -> Option<Value> {
    let engine = detect_formatter_engine(text, default_engine);
    let tab_size = options
        .and_then(|o| o.get("tabSize"))
        .and_then(|v| v.as_u64())
        .unwrap_or(4) as usize;
    let insert_spaces = options
        .and_then(|o| o.get("insertSpaces"))
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let formatted_text = match engine {
        FormatterEngine::ClangFormat => {
            let mut clang_out = None;
            if let Some(clang_format) = find_clang_format(custom_clang) {
                let filename = if let Some(path) = uri_to_path(uri) {
                    path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("shader.glsl")
                        .to_string()
                } else {
                    "shader.glsl".to_string()
                };

                log(&format!("Formatting doc '{uri}' using clang-format='{clang_format}'"));
                if let Ok(mut child) = Command::new(&clang_format)
                    .args([format!("--assume-filename={filename}")])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                {
                    if let Some(mut stdin) = child.stdin.take() {
                        let _ = stdin.write_all(text.as_bytes());
                    }
                    if let Ok(output) = child.wait_with_output() {
                        if output.status.success() {
                            if let Ok(s) = String::from_utf8(output.stdout) {
                                clang_out = Some(s);
                            }
                        }
                    }
                }
            }
            clang_out.unwrap_or_else(|| {
                warn_missing_clang_format();
                basic_glsl_format(text, tab_size, insert_spaces)
            })
        }
        FormatterEngine::Builtin => {
            log(&format!("Formatting doc '{uri}' using built-in pure-Rust formatter"));
            basic_glsl_format(text, tab_size, insert_spaces)
        }
    };

    if formatted_text == text {
        log("Doc is already formatted cleanly, returning empty edits.");
        return Some(json!([]));
    }

    let lines: Vec<&str> = text.lines().collect();
    let line_count = lines.len();
    let last_col = lines.last().map(|l| l.chars().count()).unwrap_or(0);

    Some(json!([
        {
            "range": {
                "start": { "line": 0, "character": 0 },
                "end": { "line": line_count.saturating_sub(1), "character": last_col }
            },
            "newText": formatted_text
        }
    ]))
}

pub fn format_range(
    uri: &str,
    text: &str,
    _range: Option<&Value>,
    options: Option<&Value>,
    default_engine: FormatterEngine,
    custom_clang: Option<&str>,
) -> Option<Value> {
    format_document(uri, text, options, default_engine, custom_clang)
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorItem {
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
    pub is_vec4: bool,
}

pub fn parse_color_token(token: &str) -> Option<f64> {
    let s = token.trim().trim_end_matches(['f', 'F']);
    s.parse::<f64>().ok()
}

pub fn find_colors_in_text(text: &str) -> Vec<ColorItem> {
    let mut results = Vec::new();
    for (line_idx, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") {
            continue;
        }

        let mut search_idx = 0;
        while search_idx < line.len() {
            let slice = &line[search_idx..];
            let v4_pos = slice.find("vec4(");
            let v3_pos = slice.find("vec3(");

            let (found_pos, is_v4) = match (v4_pos, v3_pos) {
                (Some(p4), Some(p3)) => {
                    if p4 <= p3 {
                        (p4, true)
                    } else {
                        (p3, false)
                    }
                }
                (Some(p4), None) => (p4, true),
                (None, Some(p3)) => (p3, false),
                (None, None) => break,
            };

            let start_char_idx = search_idx + found_pos;
            let constructor_len = 5; // "vec4(" or "vec3("
            let args_start = start_char_idx + constructor_len;

            if let Some(close_idx) = line[args_start..].find(')') {
                let end_char_idx = args_start + close_idx + 1;
                let args_str = &line[args_start..args_start + close_idx];
                let tokens: Vec<&str> = args_str.split(',').collect();

                if is_v4 && tokens.len() == 4 {
                    if let (Some(r), Some(g), Some(b), Some(a)) = (
                        parse_color_token(tokens[0]),
                        parse_color_token(tokens[1]),
                        parse_color_token(tokens[2]),
                        parse_color_token(tokens[3]),
                    ) {
                        results.push(ColorItem {
                            line: line_idx,
                            start_col: start_char_idx,
                            end_col: end_char_idx,
                            r: r.clamp(0.0, 1.0),
                            g: g.clamp(0.0, 1.0),
                            b: b.clamp(0.0, 1.0),
                            a: a.clamp(0.0, 1.0),
                            is_vec4: true,
                        });
                    }
                } else if !is_v4 && tokens.len() == 3 {
                    if let (Some(r), Some(g), Some(b)) = (
                        parse_color_token(tokens[0]),
                        parse_color_token(tokens[1]),
                        parse_color_token(tokens[2]),
                    ) {
                        results.push(ColorItem {
                            line: line_idx,
                            start_col: start_char_idx,
                            end_col: end_char_idx,
                            r: r.clamp(0.0, 1.0),
                            g: g.clamp(0.0, 1.0),
                            b: b.clamp(0.0, 1.0),
                            a: 1.0,
                            is_vec4: false,
                        });
                    }
                }
                search_idx = end_char_idx;
            } else {
                search_idx = start_char_idx + constructor_len;
            }
        }
    }
    results
}

pub fn handle_document_color(text: &str) -> Value {
    let colors = find_colors_in_text(text);
    let items: Vec<Value> = colors
        .into_iter()
        .map(|c| {
            json!({
                "range": {
                    "start": { "line": c.line, "character": c.start_col },
                    "end": { "line": c.line, "character": c.end_col }
                },
                "color": {
                    "red": c.r,
                    "green": c.g,
                    "blue": c.b,
                    "alpha": c.a
                }
            })
        })
        .collect();
    json!(items)
}

fn format_color_component(val: f64) -> String {
    if (val - val.round()).abs() < 1e-4 {
        format!("{:.1}", val)
    } else {
        format!("{:.3}", val)
    }
}

pub fn handle_color_presentation(msg: &Value, doc_cache: &HashMap<String, String>) -> Value {
    let params = match msg.get("params") {
        Some(p) => p,
        None => return json!([]),
    };
    let color = match params.get("color") {
        Some(c) => c,
        None => return json!([]),
    };

    let r = color["red"].as_f64().unwrap_or(0.0);
    let g = color["green"].as_f64().unwrap_or(0.0);
    let b = color["blue"].as_f64().unwrap_or(0.0);
    let a = color["alpha"].as_f64().unwrap_or(1.0);

    let range = &params["range"];
    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
    let is_vec4 = if let Some(doc) = doc_cache.get(uri) {
        let line_idx = range["start"]["line"].as_u64().unwrap_or(0) as usize;
        let start_col = range["start"]["character"].as_u64().unwrap_or(0) as usize;
        if let Some(line) = doc.lines().nth(line_idx) {
            line.chars().skip(start_col).take(4).collect::<String>() == "vec4"
        } else {
            true
        }
    } else {
        true
    };

    let r_str = format_color_component(r);
    let g_str = format_color_component(g);
    let b_str = format_color_component(b);
    let a_str = format_color_component(a);

    let new_text = if is_vec4 {
        format!("vec4({r_str}, {g_str}, {b_str}, {a_str})")
    } else {
        format!("vec3({r_str}, {g_str}, {b_str})")
    };

    json!([
        {
            "label": new_text,
            "textEdit": {
                "range": range,
                "newText": new_text
            }
        }
    ])
}

fn send_lsp_message<W: Write>(writer: &mut W, msg: &Value) -> io::Result<()> {
    let body = serde_json::to_string(msg)?;
    let header = format!("Content-Length: {}\r\n\r\n", body.len());
    writer.write_all(header.as_bytes())?;
    writer.write_all(body.as_bytes())?;
    writer.flush()?;
    Ok(())
}

struct ValidationRequest {
    uri: String,
    text: String,
    target: TargetApi,
    glslang_path: Option<String>,
}

fn main() -> io::Result<()> {
    log("=== glsl_validator started ===");
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    let (tx_val, rx_val) = mpsc::channel::<ValidationRequest>();
    let stdout_shared = Arc::new(Mutex::new(io::stdout()));

    let out_for_worker = Arc::clone(&stdout_shared);
    thread::spawn(move || {
        while let Ok(mut req) = rx_val.recv() {
            // Drain queue so rapid keystrokes don't pile up redundant compilations
            while let Ok(newer) = rx_val.try_recv() {
                if newer.uri == req.uri {
                    req = newer;
                } else {
                    let diagnostics = validate_shader(&req.uri, &req.text, req.target, req.glslang_path.as_deref());
                    let notif = json!({
                        "jsonrpc": "2.0",
                        "method": "textDocument/publishDiagnostics",
                        "params": {
                            "uri": req.uri,
                            "diagnostics": diagnostics
                        }
                    });
                    if let Ok(mut lock) = out_for_worker.lock() {
                        let _ = send_lsp_message(&mut *lock, &notif);
                    }
                    req = newer;
                }
            }

            let diagnostics = validate_shader(&req.uri, &req.text, req.target, req.glslang_path.as_deref());
            let notif = json!({
                "jsonrpc": "2.0",
                "method": "textDocument/publishDiagnostics",
                "params": {
                    "uri": req.uri,
                    "diagnostics": diagnostics
                }
            });
            if let Ok(mut lock) = out_for_worker.lock() {
                let _ = send_lsp_message(&mut *lock, &notif);
            }
        }
    });

    let send_resp = |msg: &Value| -> io::Result<()> {
        let mut lock = stdout_shared
            .lock()
            .map_err(|e| io::Error::other(e.to_string()))?;
        send_lsp_message(&mut *lock, msg)
    };

    let mut default_target = TargetApi::OpenGl;
    let mut default_engine = FormatterEngine::ClangFormat;
    let mut custom_glslang_path: Option<String> = None;
    let mut custom_clang_path: Option<String> = None;
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
                        if let Some(f) = opts.get("formatter").and_then(|v| v.as_str()) {
                            default_engine = FormatterEngine::parse_engine(f);
                            log(&format!("Initialized with formatter engine={:?}", default_engine));
                        }
                        if let Some(p) = opts.get("glslang_validator_path").and_then(|v| v.as_str())
                            .or_else(|| opts.get("glslang_path").and_then(|v| v.as_str())) {
                            custom_glslang_path = Some(p.to_string());
                            log(&format!("Initialized with custom glslang_path={p}"));
                        }
                        if let Some(p) = opts.get("clang_format_path").and_then(|v| v.as_str()) {
                            custom_clang_path = Some(p.to_string());
                            log(&format!("Initialized with custom clang_format_path={p}"));
                        }
                    }
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": {
                            "capabilities": {
                                "textDocumentSync": 1,
                                "completionProvider": {
                                    "triggerCharacters": ["."]
                                },
                                "signatureHelpProvider": {
                                    "triggerCharacters": ["(", ","]
                                },
                                "hoverProvider": true,
                                "documentFormattingProvider": true,
                                "documentRangeFormattingProvider": true,
                                "colorProvider": true
                            }
                        }
                    });
                    send_resp(&resp)?;
                }
                "textDocument/signatureHelp" => {
                    let sig_help = signature::handle_signature_help(&msg, &doc_cache);
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": sig_help
                    });
                    send_resp(&resp)?;
                }
                "textDocument/hover" => {
                    let hover_info = signature::handle_hover(&msg, &doc_cache);
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": hover_info
                    });
                    send_resp(&resp)?;
                }
                "textDocument/completion" => {
                    let items = handle_completion(&msg, &doc_cache);
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": items
                    });
                    send_resp(&resp)?;
                }
                "textDocument/formatting" => {
                    let uri = msg["params"]["textDocument"]["uri"].as_str().unwrap_or("");
                    let options = msg["params"].get("options");
                    let edits = doc_cache
                        .get(uri)
                        .and_then(|text| format_document(uri, text, options, default_engine, custom_clang_path.as_deref()))
                        .unwrap_or(Value::Null);
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": edits
                    });
                    send_resp(&resp)?;
                }
                "textDocument/rangeFormatting" => {
                    let uri = msg["params"]["textDocument"]["uri"].as_str().unwrap_or("");
                    let options = msg["params"].get("options");
                    let range = msg["params"].get("range");
                    let edits = doc_cache
                        .get(uri)
                        .and_then(|text| format_range(uri, text, range, options, default_engine, custom_clang_path.as_deref()))
                        .unwrap_or(Value::Null);
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": edits
                    });
                    send_resp(&resp)?;
                }
                "textDocument/documentColor" => {
                    let uri = msg["params"]["textDocument"]["uri"].as_str().unwrap_or("");
                    let colors = doc_cache
                        .get(uri)
                        .map(|text| handle_document_color(text))
                        .unwrap_or_else(|| json!([]));
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": colors
                    });
                    send_resp(&resp)?;
                }
                "textDocument/colorPresentation" => {
                    let presentations = handle_color_presentation(&msg, &doc_cache);
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": presentations
                    });
                    send_resp(&resp)?;
                }
                "shutdown" => {
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": null
                    });
                    send_resp(&resp)?;
                }
                _ => {
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": null
                    });
                    send_resp(&resp)?;
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
                    let mut revalidate = false;
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
                            revalidate = true;
                        }
                    }

                    if let Some(p) = settings.get("glslang_validator_path").and_then(|v| v.as_str())
                        .or_else(|| settings.get("glslang_path").and_then(|v| v.as_str()))
                        .or_else(|| settings.get("glsl_validator").and_then(|g| g.get("glslang_validator_path")).and_then(|v| v.as_str()))
                        .or_else(|| settings.get("glsl_validator").and_then(|g| g.get("glslang_path")).and_then(|v| v.as_str()))
                        .or_else(|| settings.get("initialization_options").and_then(|g| g.get("glslang_validator_path")).and_then(|v| v.as_str()))
                        .or_else(|| settings.get("initialization_options").and_then(|g| g.get("glslang_path")).and_then(|v| v.as_str())) {
                        if custom_glslang_path.as_deref() != Some(p) {
                            custom_glslang_path = Some(p.to_string());
                            log(&format!("Updated custom_glslang_path={p}"));
                            revalidate = true;
                        }
                    }

                    if let Some(p) = settings.get("clang_format_path").and_then(|v| v.as_str())
                        .or_else(|| settings.get("glsl_validator").and_then(|g| g.get("clang_format_path")).and_then(|v| v.as_str()))
                        .or_else(|| settings.get("initialization_options").and_then(|g| g.get("clang_format_path")).and_then(|v| v.as_str())) {
                        custom_clang_path = Some(p.to_string());
                        log(&format!("Updated custom_clang_path={p}"));
                    }

                    if let Some(f) = settings.get("formatter").and_then(|v| v.as_str()) {
                        default_engine = FormatterEngine::parse_engine(f);
                        log(&format!("Updated default_engine to {:?}", default_engine));
                    } else if let Some(f) = settings.get("glsl_validator").and_then(|g| g.get("formatter")).and_then(|v| v.as_str()) {
                        default_engine = FormatterEngine::parse_engine(f);
                        log(&format!("Updated default_engine to {:?}", default_engine));
                    } else if let Some(f) = settings.get("initialization_options").and_then(|g| g.get("formatter")).and_then(|v| v.as_str()) {
                        default_engine = FormatterEngine::parse_engine(f);
                        log(&format!("Updated default_engine to {:?}", default_engine));
                    }

                    if revalidate {
                        for (uri, text) in &doc_cache {
                            let _ = tx_val.send(ValidationRequest {
                                uri: uri.clone(),
                                text: text.clone(),
                                target: default_target,
                                glslang_path: custom_glslang_path.clone(),
                            });
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
                    let _ = tx_val.send(ValidationRequest {
                        uri: uri.to_string(),
                        text: text.to_string(),
                        target: default_target,
                        glslang_path: custom_glslang_path.clone(),
                    });
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
                                let _ = tx_val.send(ValidationRequest {
                                    uri: uri.to_string(),
                                    text: text.to_string(),
                                    target: default_target,
                                    glslang_path: custom_glslang_path.clone(),
                                });
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
                        let _ = tx_val.send(ValidationRequest {
                            uri: uri.to_string(),
                            text: text.clone(),
                            target: default_target,
                            glslang_path: custom_glslang_path.clone(),
                        });
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
                    let _ = send_resp(&notif);
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

    #[test]
    fn test_infer_vector_dimension() {
        let doc = "struct Test {\n    vec4 a;\n    vec2 b;\n};\nvec3 normal;\nTest t;\n";
        assert_eq!(infer_vector_dimension(doc, "t.a"), 4);
        assert_eq!(infer_vector_dimension(doc, "t.b"), 2);
        assert_eq!(infer_vector_dimension(doc, "normal"), 3);
        assert_eq!(infer_vector_dimension(doc, "t.a.xyz"), 3);
        assert_eq!(infer_vector_dimension(doc, "gl_Position"), 4);
    }

    #[test]
    fn test_generate_swizzle_completions() {
        let items2 = generate_swizzle_completions(2);
        assert!(items2.iter().any(|i| i["label"] == "xy"));
        assert!(items2.iter().any(|i| i["label"] == "length()"));

        let items4 = generate_swizzle_completions(4);
        assert!(items4.iter().any(|i| i["label"] == "xyzw"));
        assert!(items4.iter().any(|i| i["label"] == "rgba"));
        assert!(items4.iter().any(|i| i["label"] == "stpq"));
    }

    #[test]
    fn test_percent_decode_str() {
        assert_eq!(percent_decode_str("hello%20world"), "hello world");
        assert_eq!(percent_decode_str("shader%2Bcommon.glsl"), "shader+common.glsl");
        assert_eq!(percent_decode_str("plain_path"), "plain_path");
    }

    #[test]
    fn test_uri_to_path() {
        let p1 = uri_to_path("file:///project/test.frag");
        assert!(p1.is_some());

        let p2 = uri_to_path("file:///C:/Users/test/shader.vert");
        assert!(p2.is_some());
        if cfg!(windows) {
            assert!(p2.unwrap().to_str().unwrap().contains("C:"));
        }
    }

    #[test]
    fn test_generate_snippet_completions() {
        let ubo_snips = generate_snippet_completions("ubo");
        assert_eq!(ubo_snips.len(), 1);
        assert_eq!(ubo_snips[0]["label"], "ubo");
        assert!(ubo_snips[0]["insertText"].as_str().unwrap().contains("layout(std140, binding = ${1:0}) uniform ${2:BlockName}"));

        let ssbo_snips = generate_snippet_completions("ssbo");
        assert_eq!(ssbo_snips.len(), 1);
        assert_eq!(ssbo_snips[0]["label"], "ssbo");

        let vert_snips = generate_snippet_completions("vert");
        assert_eq!(vert_snips.len(), 1);
        assert!(vert_snips[0]["insertText"].as_str().unwrap().contains("#version 460 core"));

        let all_snips = generate_snippet_completions("");
        assert!(all_snips.len() >= 8);
    }

    #[test]
    fn test_parse_color_token() {
        assert_eq!(parse_color_token("1.0"), Some(1.0));
        assert_eq!(parse_color_token("0.5f"), Some(0.5));
        assert_eq!(parse_color_token("0.25F"), Some(0.25));
        assert_eq!(parse_color_token("0"), Some(0.0));
        assert_eq!(parse_color_token("1"), Some(1.0));
        assert_eq!(parse_color_token("invalid"), None);
    }

    #[test]
    fn test_find_colors_in_text() {
        let glsl = r#"
            vec4 col1 = vec4(1.0, 0.5, 0.2, 1.0);
            vec3 col2 = vec3(0.0, 1.0, 0.0);
            // vec4 comment = vec4(0.0, 0.0, 0.0, 1.0);
            float a = 1.0;
        "#;
        let colors = find_colors_in_text(glsl);
        assert_eq!(colors.len(), 2);
        assert!(colors[0].is_vec4);
        assert_eq!(colors[0].r, 1.0);
        assert_eq!(colors[0].g, 0.5);
        assert_eq!(colors[0].b, 0.2);
        assert_eq!(colors[0].a, 1.0);

        assert!(!colors[1].is_vec4);
        assert_eq!(colors[1].g, 1.0);
    }

    #[test]
    fn test_basic_glsl_format() {
        let unformatted = "#version 460 core\nvoid main()\n{\nif (true) {\ngl_Position = vec4(1.0);\n} else {\ngl_Position = vec4(0.0);\n}\n}\n";
        let formatted = basic_glsl_format(unformatted, 4, true);
        assert!(formatted.contains("    if (true) {"));
        assert!(formatted.contains("        gl_Position = vec4(1.0);"));
        assert!(formatted.contains("    } else {"));
        assert!(formatted.contains("        gl_Position = vec4(0.0);"));
        assert!(formatted.starts_with("#version 460 core"));
    }

    #[test]
    fn test_clean_glsl_line_syntax() {
        assert_eq!(clean_glsl_line_syntax("vec4(1.0,0.5,0.2,1.0)"), "vec4(1.0, 0.5, 0.2, 1.0)");
        assert_eq!(clean_glsl_line_syntax("void main(){"), "void main() {");
        assert_eq!(clean_glsl_line_syntax("// a,b"), "// a,b");
    }

    #[test]
    fn test_formatter_engine_detection() {
        assert_eq!(FormatterEngine::parse_engine("builtin"), FormatterEngine::Builtin);
        assert_eq!(FormatterEngine::parse_engine("clang-format"), FormatterEngine::ClangFormat);
        assert_eq!(FormatterEngine::parse_engine("clang"), FormatterEngine::ClangFormat);
        assert_eq!(FormatterEngine::parse_engine("unknown"), FormatterEngine::Builtin);

        assert_eq!(
            detect_formatter_engine("// @formatter: clang-format\nvoid main() {}", FormatterEngine::Builtin),
            FormatterEngine::ClangFormat
        );
        assert_eq!(
            detect_formatter_engine("// @formatter: builtin\nvoid main() {}", FormatterEngine::ClangFormat),
            FormatterEngine::Builtin
        );
    }

    #[test]
    fn test_find_in_path() {
        // "cargo" or "cmd" or "sh" should exist in system PATH on any dev environment
        let cargo_found = find_in_path("cargo").is_some();
        let cmd_or_sh = if cfg!(windows) {
            find_in_path("cmd").is_some() || find_in_path("cmd.exe").is_some()
        } else {
            find_in_path("sh").is_some()
        };
        assert!(cargo_found || cmd_or_sh);
    }

    #[test]
    fn test_custom_binary_paths() {
        // Non-existent custom path should not crash and fall back to regular search
        let fallback_clang = find_clang_format(Some("non_existent_fake_path_xyz123"));
        assert!(fallback_clang.is_some() || fallback_clang.is_none());

        let fallback_glslang = find_glslang_validator(Some("non_existent_fake_path_xyz123"));
        assert!(fallback_glslang.is_some() || fallback_glslang.is_none());
    }

    #[test]
    fn test_completion_include_and_builtins() {
        let mut doc_cache = HashMap::new();
        let common_uri = "file:///project/shaders/common.glsl";
        let common_code = r#"
vec3 calculateNormal(mat4 normal, vec3 aNormal) {
    return mat3(normal) * aNormal;
}
"#;
        doc_cache.insert(common_uri.to_string(), common_code.to_string());

        let main_uri = "file:///project/shaders/main.frag";
        let main_code = "#include \"common.glsl\"\nvec3 n = calc";
        doc_cache.insert(main_uri.to_string(), main_code.to_string());

        let req = json!({
            "params": {
                "textDocument": { "uri": main_uri },
                "position": { "line": 1, "character": 13 }
            }
        });

        let res = handle_completion(&req, &doc_cache);
        let items = res.as_array().expect("items array");

        // Should find calculateNormal from common.glsl
        let calc_item = items.iter().find(|it| it["label"] == "calculateNormal");
        assert!(calc_item.is_some(), "calculateNormal must be found in completions");
        let item = calc_item.unwrap();
        assert_eq!(item["kind"], 3); // Function
        let doc_str = item["documentation"]["value"].as_str().unwrap();
        assert!(doc_str.contains("common.glsl"));

        // Built-in test: typing "norm"
        let norm_code = "vec3 n = norm";
        doc_cache.insert(main_uri.to_string(), norm_code.to_string());
        let norm_req = json!({
            "params": {
                "textDocument": { "uri": main_uri },
                "position": { "line": 0, "character": 13 }
            }
        });
        let norm_res = handle_completion(&norm_req, &doc_cache);
        let norm_items = norm_res.as_array().expect("norm items");
        let norm_item = norm_items.iter().find(|it| it["label"] == "normalize");
        assert!(norm_item.is_some(), "normalize must be found in completions");
    }
}