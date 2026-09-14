// glsl_validator - signature.rs
// Signature Help and Hover provider for GLSL 4.6 (built-ins from docs.gl & user functions across #include)

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use serde_json::{json, Value};
use crate::docs;
use crate::uri_to_path;

const INVALID_TYPES: &[&str] = &[
    "return", "else", "case", "default", "discard", "break", "continue", "goto", "layout", "precision",
];
const INVALID_NAMES: &[&str] = &[
    "if", "for", "while", "switch", "return", "layout", "struct", "subroutine",
];
const CONTROL_KEYWORDS: &[&str] = &[
    "if", "for", "while", "switch", "catch", "return",
];

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub return_type: String,
    pub label: String,
    pub parameters: Vec<String>,
    pub doc: Option<String>,
    pub source: Option<String>,
}

/// Identifies the function call surrounding the cursor and the active parameter index.
/// CRLF-safe, zero-allocation byte scanning.
pub fn find_enclosing_call(text: &str, line_idx: usize, col_idx: usize) -> Option<(String, usize)> {
    let mut current_line = 0;
    let mut offset = text.len();
    let mut line_start = 0;

    for (idx, b) in text.bytes().enumerate() {
        if current_line == line_idx {
            line_start = idx;
            break;
        }
        if b == b'\n' {
            current_line += 1;
        }
    }

    if current_line == line_idx {
        let line_slice = &text[line_start..];
        let raw_line_len = line_slice.find('\n').unwrap_or(line_slice.len());
        let line_len = line_slice[..raw_line_len].trim_end_matches('\r').len();
        offset = line_start + col_idx.min(line_len);
    }

    let bytes = text.as_bytes();
    let mut depth = 0;
    let mut bracket_depth = 0;
    let mut brace_depth = 0;
    let mut open_paren_idx = None;

    // Scan backwards from cursor
    let mut idx = offset;
    while idx > 0 {
        idx -= 1;
        let b = bytes[idx];

        // Skip string literals if any
        if b == b'"' {
            while idx > 0 {
                idx -= 1;
                if bytes[idx] == b'"' && (idx == 0 || bytes[idx - 1] != b'\\') {
                    break;
                }
            }
            continue;
        }

        match b {
            b')' => depth += 1,
            b']' => bracket_depth += 1,
            b'}' => brace_depth += 1,
            b'(' => {
                if depth > 0 {
                    depth -= 1;
                } else if bracket_depth == 0 && brace_depth == 0 {
                    open_paren_idx = Some(idx);
                    break;
                }
            }
            b'[' => {
                if bracket_depth > 0 {
                    bracket_depth -= 1;
                }
            }
            b'{' => {
                if brace_depth > 0 {
                    brace_depth -= 1;
                } else {
                    // Reached end of statement / block boundary
                    break;
                }
            }
            b';' if depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                // Reached statement separator
                break;
            }
            _ => {}
        }
    }

    let open_idx = open_paren_idx?;

    // Extract function identifier before '('
    let before_paren = &text[..open_idx];
    let trimmed = before_paren.trim_end();
    let mut ident_start = trimmed.len();
    for (i, c) in trimmed.char_indices().rev() {
        if c.is_alphanumeric() || c == '_' {
            ident_start = i;
        } else {
            break;
        }
    }

    let fn_name = trimmed[ident_start..].trim();
    if fn_name.is_empty() || CONTROL_KEYWORDS.contains(&fn_name) {
        return None;
    }

    // Now count commas between open_paren and cursor_offset at top level
    let mut active_param = 0;
    let mut inner_paren = 0;
    let mut inner_bracket = 0;
    let mut inner_brace = 0;

    let between = &bytes[open_idx + 1..offset];
    for &b in between {
        match b {
            b'(' => inner_paren += 1,
            b')' => {
                if inner_paren > 0 {
                    inner_paren -= 1;
                }
            }
            b'[' => inner_bracket += 1,
            b']' => {
                if inner_bracket > 0 {
                    inner_bracket -= 1;
                }
            }
            b'{' => inner_brace += 1,
            b'}' => {
                if inner_brace > 0 {
                    inner_brace -= 1;
                }
            }
            b',' if inner_paren == 0 && inner_bracket == 0 && inner_brace == 0 => {
                active_param += 1;
            }
            _ => {}
        }
    }

    Some((fn_name.to_string(), active_param))
}

fn parse_function_header(
    header: &str,
    pending_doc: &[String],
    source_name: Option<&str>,
) -> Option<FunctionSignature> {
    let open_paren = header.find('(')?;
    let close_paren = header.rfind(')')?;
    if close_paren <= open_paren {
        return None;
    }

    let before = header[..open_paren].trim();
    let tokens: Vec<&str> = before.split_whitespace().collect();
    if tokens.len() < 2 {
        return None;
    }

    let fn_name = tokens.last()?.to_string();
    let return_type = tokens[tokens.len() - 2].to_string();

    let params_str = header[open_paren + 1..close_paren].trim();
    let parameters: Vec<String> = if params_str.is_empty() || params_str == "void" {
        Vec::new()
    } else {
        params_str
            .split(',')
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect()
    };

    let param_joined = parameters.join(", ");
    let label = format!("{return_type} {fn_name}({param_joined})");
    let doc = if !pending_doc.is_empty() {
        Some(pending_doc.join("\n"))
    } else {
        None
    };

    Some(FunctionSignature {
        name: fn_name,
        return_type,
        label,
        parameters,
        doc,
        source: source_name.map(|s| s.to_string()),
    })
}

/// Parses function signatures from GLSL source text without allocating an entire lines vector.
pub fn scan_user_functions(text: &str, source_name: Option<&str>) -> Vec<FunctionSignature> {
    let mut results = Vec::new();
    let mut pending_doc = Vec::new();
    let mut brace_level: usize = 0;
    let mut multiline_header: Option<String> = None;

    for raw_line in text.lines() {
        let line = raw_line.trim();

        // Handle multi-line function declaration continuation
        if let Some(mut header) = multiline_header.take() {
            header.push(' ');
            header.push_str(line);

            if let Some(close_idx) = header.find(')') {
                if let Some(sig) = parse_function_header(&header[..=close_idx], &pending_doc, source_name) {
                    results.push(sig);
                }
                pending_doc.clear();
            } else {
                multiline_header = Some(header);
            }

            for b in line.bytes() {
                if b == b'{' {
                    brace_level += 1;
                } else if b == b'}' {
                    brace_level = brace_level.saturating_sub(1);
                }
            }
            continue;
        }

        // Collect doc comments at top level
        if line.starts_with("//") {
            if brace_level == 0 {
                let doc_line = line.trim_start_matches('/').trim();
                pending_doc.push(doc_line.to_string());
            }
            continue;
        }

        if line.is_empty() {
            if brace_level == 0 {
                pending_doc.clear();
            }
            continue;
        }

        // Only scan function declarations/definitions at global scope
        if brace_level == 0 && !line.starts_with("return") && !line.starts_with('#') {
            if let Some(open_paren) = line.find('(') {
                let before = line[..open_paren].trim();
                let tokens: Vec<&str> = before.split_whitespace().collect();

                if tokens.len() >= 2 {
                    let fn_name = tokens.last().unwrap();
                    let return_type = tokens[tokens.len() - 2];

                    let is_valid_name = !fn_name.is_empty()
                        && (fn_name.starts_with(|c: char| c.is_alphabetic() || c == '_'))
                        && fn_name.chars().all(|c| c.is_alphanumeric() || c == '_');

                    if is_valid_name && !INVALID_NAMES.contains(fn_name) && !INVALID_TYPES.contains(&return_type) {
                        if let Some(close_idx) = line.find(')') {
                            if let Some(sig) = parse_function_header(&line[..=close_idx], &pending_doc, source_name) {
                                results.push(sig);
                            }
                            pending_doc.clear();
                        } else {
                            multiline_header = Some(line.to_string());
                        }
                    }
                }
            }
        }

        // Track braces on this line to maintain scope
        for b in line.bytes() {
            if b == b'{' {
                brace_level += 1;
            } else if b == b'}' {
                brace_level = brace_level.saturating_sub(1);
            }
        }

        if brace_level == 0 && multiline_header.is_none() {
            pending_doc.clear();
        }
    }

    results
}

/// Resolves #include directives and aggregates function signatures.
pub fn resolve_includes_and_scan(
    uri: &str,
    text: &str,
    doc_cache: &HashMap<String, String>,
) -> Vec<FunctionSignature> {
    let mut all_functions = scan_user_functions(text, None);
    let mut visited: HashSet<PathBuf> = HashSet::new();

    let base_dir = uri_to_path(uri).and_then(|p| p.parent().map(|dir| dir.to_path_buf()));

    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("#include") {
            let include_target = rest.trim().trim_matches(['"', '<', '>']);
            if include_target.is_empty() {
                continue;
            }

            if let Some(ref dir) = base_dir {
                let candidate = dir.join(include_target);
                if !visited.insert(candidate.clone()) {
                    continue;
                }

                // Check doc_cache by file:/// URI first (0 disk I/O, live unsaved buffer)
                let candidate_uri = format!("file:///{}", candidate.to_string_lossy().replace('\\', "/"));
                let content = doc_cache.get(&candidate_uri).map(|cached| cached.as_str());

                let disk_content;
                let text_ref = match content {
                    Some(c) => Some(c),
                    None => {
                        if candidate.is_file() {
                            disk_content = std::fs::read_to_string(&candidate).ok();
                            disk_content.as_deref()
                        } else {
                            None
                        }
                    }
                };

                if let Some(content_text) = text_ref {
                    let source_label = candidate
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(include_target);
                    let inc_funcs = scan_user_functions(content_text, Some(source_label));
                    all_functions.extend(inc_funcs);
                }
            }
        }
    }

    all_functions
}

/// Handles textDocument/signatureHelp requests.
pub fn handle_signature_help(msg: &Value, doc_cache: &HashMap<String, String>) -> Value {
    let params = match msg.get("params") {
        Some(p) => p,
        None => return json!(null),
    };

    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
    let line_idx = params["position"]["line"].as_u64().unwrap_or(0) as usize;
    let col_idx = params["position"]["character"].as_u64().unwrap_or(0) as usize;

    let doc = match doc_cache.get(uri) {
        Some(d) => d,
        None => return json!(null),
    };

    let (fn_name, active_param) = match find_enclosing_call(doc, line_idx, col_idx) {
        Some(call) => call,
        None => return json!(null),
    };

    // 1. Fast lookup: Check built-in functions (docs.gl) - takes microseconds, 0 disk I/O
    if let Some(builtin) = docs::lookup_builtin_function(&fn_name) {
        let mut signatures = Vec::with_capacity(builtin.overloads.len());
        let mut active_sig = 0;

        for (idx, overload) in builtin.overloads.iter().enumerate() {
            let params_json: Vec<Value> = overload
                .params
                .iter()
                .map(|p| json!({ "label": p }))
                .collect();

            signatures.push(json!({
                "label": overload.label,
                "documentation": {
                    "kind": "markdown",
                    "value": builtin.description
                },
                "parameters": params_json
            }));

            // Pick overload matching argument count if possible
            if active_param < overload.params.len() {
                active_sig = idx;
            }
        }

        return json!({
            "signatures": signatures,
            "activeSignature": active_sig,
            "activeParameter": active_param
        });
    }

    // 2. Check user-defined functions (current doc + #includes)
    let user_funcs = resolve_includes_and_scan(uri, doc, doc_cache);
    let matched_funcs: Vec<&FunctionSignature> = user_funcs.iter().filter(|f| f.name == fn_name).collect();

    if !matched_funcs.is_empty() {
        let mut signatures = Vec::with_capacity(matched_funcs.len());
        let mut active_sig = 0;

        for (idx, func) in matched_funcs.iter().enumerate() {
            let params_json: Vec<Value> = func
                .parameters
                .iter()
                .map(|p| json!({ "label": p }))
                .collect();

            let doc_text = match (&func.doc, &func.source) {
                (Some(d), Some(src)) => format!("**Source:** `{src}`\n\n{d}"),
                (Some(d), None) => d.clone(),
                (None, Some(src)) => format!("**Source:** `{src}`"),
                (None, None) => format!("User-defined function `{}`", func.name),
            };

            signatures.push(json!({
                "label": func.label,
                "documentation": {
                    "kind": "markdown",
                    "value": doc_text
                },
                "parameters": params_json
            }));

            if active_param < func.parameters.len() {
                active_sig = idx;
            }
        }

        return json!({
            "signatures": signatures,
            "activeSignature": active_sig,
            "activeParameter": active_param
        });
    }

    json!(null)
}

/// Handles textDocument/hover requests for built-ins and user-defined functions.
pub fn handle_hover(msg: &Value, doc_cache: &HashMap<String, String>) -> Value {
    let params = match msg.get("params") {
        Some(p) => p,
        None => return json!(null),
    };

    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
    let line_idx = params["position"]["line"].as_u64().unwrap_or(0) as usize;
    let col_idx = params["position"]["character"].as_u64().unwrap_or(0) as usize;

    let doc = match doc_cache.get(uri) {
        Some(d) => d,
        None => return json!(null),
    };

    let line = match doc.lines().nth(line_idx) {
        Some(l) => l,
        None => return json!(null),
    };

    if col_idx > line.len() {
        return json!(null);
    }

    // Extract word under cursor
    let mut word_start = col_idx;
    for (i, c) in line[..col_idx].char_indices().rev() {
        if c.is_alphanumeric() || c == '_' {
            word_start = i;
        } else {
            break;
        }
    }

    let mut word_end = col_idx;
    for (i, c) in line[col_idx..].char_indices() {
        if c.is_alphanumeric() || c == '_' {
            word_end = col_idx + i + c.len_utf8();
        } else {
            break;
        }
    }

    let word = &line[word_start..word_end];
    if word.is_empty() {
        return json!(null);
    }

    // 1. Fast lookup: Built-in functions (docs.gl)
    if let Some(builtin) = docs::lookup_builtin_function(word) {
        let mut overloads_str = String::new();
        for ol in builtin.overloads {
            overloads_str.push_str(ol.label);
            overloads_str.push('\n');
        }

        let markdown = format!(
            "```glsl\n{}\n```\n\n{}",
            overloads_str.trim_end(),
            builtin.description
        );

        return json!({
            "contents": {
                "kind": "markdown",
                "value": markdown
            }
        });
    }

    // 2. User-defined functions
    let user_funcs = resolve_includes_and_scan(uri, doc, doc_cache);
    if let Some(func) = user_funcs.iter().find(|f| f.name == word) {
        let doc_text = func.doc.as_deref().unwrap_or("");
        let source_info = match &func.source {
            Some(src) => format!("*Defined in `{src}`*\n\n"),
            None => String::new(),
        };

        let markdown = format!(
            "```glsl\n{}\n```\n\n{}{}",
            func.label,
            source_info,
            doc_text
        );

        return json!({
            "contents": {
                "kind": "markdown",
                "value": markdown
            }
        });
    }

    json!(null)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_enclosing_call_simple() {
        let code = "vec3 res = calcualteNormal();";
        // Cursor right inside '(': index 27
        let call = find_enclosing_call(code, 0, 27);
        assert_eq!(call, Some(("calcualteNormal".to_string(), 0)));
    }

    #[test]
    fn test_find_enclosing_call_second_param() {
        let code = "vec3 res = calcualteNormal(mat4(1.0), );";
        // Cursor right after comma and space: index 38
        let call = find_enclosing_call(code, 0, 38);
        assert_eq!(call, Some(("calcualteNormal".to_string(), 1)));
    }

    #[test]
    fn test_find_enclosing_call_nested() {
        let code = "mat4 m = calculateMVP(getFragPos(model, aPos), view, proj);";
        // Cursor inside getFragPos(model, |)
        let call = find_enclosing_call(code, 0, 39);
        assert_eq!(call, Some(("getFragPos".to_string(), 1)));
    }

    #[test]
    fn test_scan_user_functions_from_user_screenshot() {
        let code = r#"
mat4 calculateMVP(mat4 model, mat4 view, mat4 projection){
    return model * view * projection;
}

vec3 getFragPos(mat4 model, vec3 aPos){
    return vec3(model * vec4(aPos, 1.0));
}

vec3 calcualteNormal(mat4 normal, vec3 aNormal){
    return mat3(normal) * aNormal;
}
"#;
        let funcs = scan_user_functions(code, None);
        assert_eq!(funcs.len(), 3);

        assert_eq!(funcs[0].name, "calculateMVP");
        assert_eq!(funcs[0].return_type, "mat4");
        assert_eq!(funcs[0].parameters.len(), 3);
        assert_eq!(funcs[0].parameters[0], "mat4 model");
        assert_eq!(funcs[0].parameters[1], "mat4 view");
        assert_eq!(funcs[0].parameters[2], "mat4 projection");

        assert_eq!(funcs[1].name, "getFragPos");
        assert_eq!(funcs[1].parameters.len(), 2);
        assert_eq!(funcs[1].parameters[0], "mat4 model");
        assert_eq!(funcs[1].parameters[1], "vec3 aPos");

        assert_eq!(funcs[2].name, "calcualteNormal");
        assert_eq!(funcs[2].parameters.len(), 2);
        assert_eq!(funcs[2].parameters[0], "mat4 normal");
        assert_eq!(funcs[2].parameters[1], "vec3 aNormal");
    }

    #[test]
    fn test_docs_builtin_lookup() {
        let norm = docs::lookup_builtin_function("normalize");
        assert!(norm.is_some());
        let norm_fn = norm.unwrap();
        assert!(norm_fn.description.contains("docs.gl"));
        assert!(norm_fn.overloads.len() >= 4);

        let dot = docs::lookup_builtin_function("dot");
        assert!(dot.is_some());

        let tex = docs::lookup_builtin_function("texture");
        assert!(tex.is_some());
    }

    #[test]
    fn test_scan_multiline_function_and_docs() {
        let code = r#"
// Calculates lighting attenuation
// based on distance.
float calculateAttenuation(
    float distance,
    float constant,
    float linear,
    float quadratic
) {
    return 1.0 / (constant + linear * distance + quadratic * distance * distance);
}
"#;
        let funcs = scan_user_functions(code, None);
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].name, "calculateAttenuation");
        assert_eq!(funcs[0].parameters.len(), 4);
        assert!(funcs[0].doc.is_some());
        assert!(funcs[0].doc.as_ref().unwrap().contains("attenuation"));
    }

    #[test]
    fn test_find_enclosing_call_crlf() {
        let code = "vec3 a = vec3(1.0);\r\nvec3 b = calcualteNormal(a, );\r\n";
        // Line 1 (0-indexed), after comma and space: col 28
        let call = find_enclosing_call(code, 1, 28);
        assert_eq!(call, Some(("calcualteNormal".to_string(), 1)));
    }
}
