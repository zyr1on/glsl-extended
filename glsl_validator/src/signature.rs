// glsl_validator - signature.rs
// Signature Help and Hover provider for GLSL 4.6 (built-ins from docs.gl & user functions across #include)

use crate::docs;
use crate::{path_to_uri, uri_to_path};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;

const INVALID_TYPES: &[&str] = &[
    "return",
    "else",
    "case",
    "default",
    "discard",
    "break",
    "continue",
    "goto",
    "layout",
    "precision",
];
const INVALID_NAMES: &[&str] = &[
    "if",
    "for",
    "while",
    "switch",
    "return",
    "layout",
    "struct",
    "subroutine",
];
const CONTROL_KEYWORDS: &[&str] = &["if", "for", "while", "switch", "catch", "return"];

pub const STORAGE_QUALIFIERS: &[&str] = &[
    "in",
    "out",
    "inout",
    "uniform",
    "buffer",
    "attribute",
    "varying",
    "const",
    "shared",
    "flat",
    "smooth",
    "noperspective",
    "centroid",
    "sample",
    "patch",
    "coherent",
    "readonly",
    "writeonly",
    "volatile",
    "restrict",
    "highp",
    "mediump",
    "lowp",
];

pub const KNOWN_BASE_TYPES: &[&str] = &[
    "float",
    "double",
    "int",
    "uint",
    "bool",
    "vec2",
    "vec3",
    "vec4",
    "dvec2",
    "dvec3",
    "dvec4",
    "bvec2",
    "bvec3",
    "bvec4",
    "ivec2",
    "ivec3",
    "ivec4",
    "uvec2",
    "uvec3",
    "uvec4",
    "mat2",
    "mat3",
    "mat4",
    "mat2x2",
    "mat2x3",
    "mat2x4",
    "mat3x2",
    "mat3x3",
    "mat3x4",
    "mat4x2",
    "mat4x3",
    "mat4x4",
    "dmat2",
    "dmat3",
    "dmat4",
    "dmat2x2",
    "dmat2x3",
    "dmat2x4",
    "dmat3x2",
    "dmat3x3",
    "dmat3x4",
    "dmat4x2",
    "dmat4x3",
    "dmat4x4",
    "sampler1D",
    "sampler2D",
    "sampler3D",
    "samplerCube",
    "sampler2DShadow",
    "samplerCubeShadow",
    "sampler2DArray",
    "sampler2DArrayShadow",
    "sampler1DArray",
    "sampler1DArrayShadow",
    "samplerCubeArray",
    "samplerCubeArrayShadow",
    "sampler2DMS",
    "sampler2DMSArray",
    "samplerBuffer",
    "isampler1D",
    "isampler2D",
    "isampler3D",
    "isamplerCube",
    "isampler2DArray",
    "isampler1DArray",
    "isamplerCubeArray",
    "isampler2DMS",
    "isampler2DMSArray",
    "isamplerBuffer",
    "usampler1D",
    "usampler2D",
    "usampler3D",
    "usamplerCube",
    "usampler2DArray",
    "usampler1DArray",
    "usamplerCubeArray",
    "usampler2DMS",
    "usampler2DMSArray",
    "usamplerBuffer",
    "image1D",
    "image2D",
    "image3D",
    "imageCube",
    "image2DArray",
    "imageCubeArray",
    "image2DMS",
    "image2DMSArray",
    "imageBuffer",
    "iimage1D",
    "iimage2D",
    "iimage3D",
    "iimageCube",
    "iimage2DArray",
    "iimageCubeArray",
    "iimage2DMS",
    "iimage2DMSArray",
    "iimageBuffer",
    "uimage1D",
    "uimage2D",
    "uimage3D",
    "uimageCube",
    "uimage2DArray",
    "uimageCubeArray",
    "uimage2DMS",
    "uimage2DMSArray",
    "uimageBuffer",
    "atomic_uint",
];

#[derive(Debug, Clone, PartialEq)]
pub struct VariableSymbol {
    pub name: String,
    pub var_type: String,
    pub qualifier: String,
    pub doc: Option<String>,
    pub source: Option<String>,
    pub line: usize,
    pub col: usize,
    pub file_uri: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub return_type: String,
    pub label: String,
    pub parameters: Vec<String>,
    pub doc: Option<String>,
    pub source: Option<String>,
    pub line: usize,
    pub col: usize,
    pub file_uri: Option<String>,
}

/// Determines if a given cursor position (line, col) is inside a comment or string literal.
/// Zero-allocation, streaming single-pass character scan.
pub fn is_in_comment_or_string(text: &str, target_line: usize, target_col: usize) -> bool {
    let mut line_idx = 0;
    let mut col_idx = 0;
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut in_string = false;
    let mut escaped = false;

    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if line_idx == target_line && col_idx >= target_col {
            return in_line_comment || in_block_comment || in_string;
        }

        if c == '\n' {
            in_line_comment = false;
            line_idx += 1;
            col_idx = 0;
            if line_idx > target_line {
                return false;
            }
            continue;
        }

        col_idx += 1;

        if in_line_comment {
            continue;
        }

        if in_block_comment {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next();
                col_idx += 1;
                in_block_comment = false;
            }
            continue;
        }

        if in_string {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }

        match c {
            '"' => in_string = true,
            '/' => {
                if chars.peek() == Some(&'/') {
                    chars.next();
                    col_idx += 1;
                    in_line_comment = true;
                } else if chars.peek() == Some(&'*') {
                    chars.next();
                    col_idx += 1;
                    in_block_comment = true;
                }
            }
            _ => {}
        }
    }

    if line_idx == target_line {
        in_line_comment || in_block_comment || in_string
    } else {
        false
    }
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
    if offset <= open_idx {
        return None;
    }

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
    line_num: usize,
    col_num: usize,
    file_uri: Option<&str>,
) -> Option<FunctionSignature> {
    let open_paren = header.find('(')?;
    let close_paren = header.rfind(')')?;
    if close_paren <= open_paren {
        return None;
    }

    let before = header[..open_paren].trim();
    let mut it = before.split_whitespace().rev();
    let fn_name = it.next()?.to_string();
    let return_type = it.next()?.to_string();

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
        line: line_num,
        col: col_num,
        file_uri: file_uri.map(|s| s.to_string()),
    })
}

/// Parses function signatures from GLSL source text without allocating an entire lines vector.
pub fn scan_user_functions(
    text: &str,
    source_name: Option<&str>,
    file_uri: Option<&str>,
) -> Vec<FunctionSignature> {
    let mut results = Vec::new();
    let mut pending_doc = Vec::new();
    let mut brace_level: usize = 0;
    let mut multiline_header: Option<(usize, usize, String)> = None;

    for (line_idx, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();

        // Handle multi-line function declaration continuation
        if let Some((start_line, start_col, mut header)) = multiline_header.take() {
            header.push(' ');
            header.push_str(line);

            if let Some(close_idx) = header.find(')') {
                if let Some(sig) = parse_function_header(
                    &header[..=close_idx],
                    &pending_doc,
                    source_name,
                    start_line,
                    start_col,
                    file_uri,
                ) {
                    results.push(sig);
                }
                pending_doc.clear();
            } else {
                multiline_header = Some((start_line, start_col, header));
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
                let mut it = before.split_whitespace().rev();
                if let (Some(fn_name), Some(return_type)) = (it.next(), it.next()) {
                    let is_valid_name = !fn_name.is_empty()
                        && (fn_name.starts_with(|c: char| c.is_alphabetic() || c == '_'))
                        && fn_name.chars().all(|c| c.is_alphanumeric() || c == '_');

                    if is_valid_name
                        && !INVALID_NAMES.contains(&fn_name)
                        && !INVALID_TYPES.contains(&return_type)
                    {
                        let col_idx = raw_line.find(fn_name).unwrap_or(0);
                        if let Some(close_idx) = line.find(')') {
                            if let Some(sig) = parse_function_header(
                                &line[..=close_idx],
                                &pending_doc,
                                source_name,
                                line_idx,
                                col_idx,
                                file_uri,
                            ) {
                                results.push(sig);
                            }
                            pending_doc.clear();
                        } else {
                            multiline_header = Some((line_idx, col_idx, line.to_string()));
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

pub fn is_valid_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) if first.is_alphabetic() || first == '_' => {
            chars.all(|c| c.is_alphanumeric() || c == '_')
        }
        _ => false,
    }
}

/// Parses user variable, constant, macro, and struct declarations from GLSL source text.
pub fn scan_user_variables(
    text: &str,
    source_name: Option<&str>,
    file_uri: Option<&str>,
) -> Vec<VariableSymbol> {
    let mut results = Vec::new();
    let mut pending_doc = Vec::new();
    let mut custom_types: HashSet<String> = HashSet::new();
    let mut brace_level: usize = 0;
    let mut current_block_type: Option<String> = None;
    let mut current_block_qualifier: String = "uniform".to_string();

    for (line_idx, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();

        // Comments
        if line.starts_with("//") {
            let doc_line = line.trim_start_matches('/').trim();
            if !doc_line.is_empty() {
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

        // Preprocessor macros: #define NAME VALUE
        if let Some(rest) = line.strip_prefix("#define") {
            let mut it = rest.split_whitespace();
            if let Some(macro_name_raw) = it.next() {
                let macro_name = macro_name_raw.split('(').next().unwrap_or(macro_name_raw);
                if is_valid_identifier(macro_name) {
                    let col = raw_line.find(macro_name).unwrap_or(0);
                    let doc_text = if pending_doc.is_empty() {
                        None
                    } else {
                        Some(pending_doc.join(" "))
                    };
                    results.push(VariableSymbol {
                        name: macro_name.to_string(),
                        var_type: "macro".to_string(),
                        qualifier: "#define".to_string(),
                        doc: doc_text,
                        source: source_name.map(|s| s.to_string()),
                        line: line_idx,
                        col,
                        file_uri: file_uri.map(|s| s.to_string()),
                    });
                }
            }
            pending_doc.clear();
            continue;
        }

        // Struct definitions: struct Name { ... }
        if let Some(struct_idx) = line.find("struct") {
            let before = &line[..struct_idx];
            let after = line[struct_idx + 6..].trim_start();
            let before_ok =
                before.is_empty() || before.chars().last().is_none_or(|c| c.is_whitespace());
            if before_ok {
                let name_candidate = after
                    .split(|c: char| c.is_whitespace() || c == '{')
                    .next()
                    .unwrap_or("");
                if is_valid_identifier(name_candidate) && !INVALID_NAMES.contains(&name_candidate) {
                    custom_types.insert(name_candidate.to_string());
                    let col = raw_line.find(name_candidate).unwrap_or(0);
                    let doc_text = if pending_doc.is_empty() {
                        None
                    } else {
                        Some(pending_doc.join(" "))
                    };
                    results.push(VariableSymbol {
                        name: name_candidate.to_string(),
                        var_type: "struct".to_string(),
                        qualifier: "struct".to_string(),
                        doc: doc_text,
                        source: source_name.map(|s| s.to_string()),
                        line: line_idx,
                        col,
                        file_uri: file_uri.map(|s| s.to_string()),
                    });
                }
            }
        }

        // Interface blocks: [layout(...)] uniform/buffer BlockName { ... } [instanceName];
        let after_layout = if let Some(layout_start) = line.find("layout") {
            if let Some(paren_close) = line[layout_start..].find(')') {
                line[layout_start + paren_close + 1..].trim()
            } else {
                line
            }
        } else {
            line
        };

        if after_layout.contains('{') {
            for qual in &["uniform", "buffer"] {
                if let Some(pos) = after_layout.find(qual) {
                    let before = &after_layout[..pos];
                    if before.is_empty() || before.chars().last().is_none_or(|c| c.is_whitespace()) {
                        let after = after_layout[pos + qual.len()..].trim_start();
                        let block_name = after
                            .split(|c: char| c.is_whitespace() || c == '{')
                            .next()
                            .unwrap_or("");
                        if is_valid_identifier(block_name) && !INVALID_NAMES.contains(&block_name) {
                            current_block_type = Some(block_name.to_string());
                            current_block_qualifier = qual.to_string();
                            custom_types.insert(block_name.to_string());
                            let col = raw_line.find(block_name).unwrap_or(0);
                            let doc_text = if pending_doc.is_empty() {
                                None
                            } else {
                                Some(pending_doc.join(" "))
                            };
                            results.push(VariableSymbol {
                                name: block_name.to_string(),
                                var_type: qual.to_string(),
                                qualifier: qual.to_string(),
                                doc: doc_text,
                                source: source_name.map(|s| s.to_string()),
                                line: line_idx,
                                col,
                                file_uri: file_uri.map(|s| s.to_string()),
                            });
                        }
                    }
                }
            }
        }

        // Track braces
        let mut open_b = 0;
        let mut close_b = 0;
        for b in line.bytes() {
            if b == b'{' {
                open_b += 1;
            } else if b == b'}' {
                close_b += 1;
            }
        }

        // Strip layout(...) if present
        let mut clean = line;
        if let Some(layout_start) = clean.find("layout") {
            if let Some(paren_close) = clean[layout_start..].find(')') {
                clean = clean[layout_start + paren_close + 1..].trim();
            }
        }

        // Strip line comments
        if let Some(c_idx) = clean.find("//") {
            clean = clean[..c_idx].trim();
        }

        // Block instance: } instanceName; or } instanceName[N];
        if let Some(brace_pos) = clean.find('}') {
            let after_brace = clean[brace_pos + 1..].trim();
            if after_brace.contains(';') {
                let inst_part = after_brace.split(';').next().unwrap_or("").trim();
                let inst_name = inst_part
                    .split(|c: char| c.is_whitespace() || c == '[')
                    .next()
                    .unwrap_or("");
                if is_valid_identifier(inst_name) && !INVALID_NAMES.contains(&inst_name) {
                    let col = raw_line.rfind(inst_name).unwrap_or(0);
                    let var_type = current_block_type
                        .clone()
                        .unwrap_or_else(|| "block".to_string());
                    let qualifier = if current_block_qualifier.is_empty() {
                        "uniform".to_string()
                    } else {
                        current_block_qualifier.clone()
                    };
                    results.push(VariableSymbol {
                        name: inst_name.to_string(),
                        var_type,
                        qualifier,
                        doc: None,
                        source: source_name.map(|s| s.to_string()),
                        line: line_idx,
                        col,
                        file_uri: file_uri.map(|s| s.to_string()),
                    });
                }
            }
            current_block_type = None;
        }

        // Check for variable declaration ending with ';' or '='
        if !clean.starts_with('#') && (clean.contains(';') || clean.contains('=')) {
            let stmt = clean.split([';', '=']).next().unwrap_or("").trim();

            let is_fn = if let Some(p) = stmt.find('(') {
                p > 0
            } else {
                false
            };

            if !is_fn && !stmt.is_empty() {
                let ctx = VarParseContext {
                    raw_line,
                    line_idx,
                    pending_doc: &pending_doc,
                    source_name,
                    file_uri,
                    custom_types: &custom_types,
                };
                parse_variable_statement(stmt, &ctx, &mut results);
            }
        }

        brace_level = (brace_level + open_b).saturating_sub(close_b);
        if line.ends_with(';') || line.ends_with('}') {
            pending_doc.clear();
        }
    }

    results
}

struct VarParseContext<'a> {
    raw_line: &'a str,
    line_idx: usize,
    pending_doc: &'a [String],
    source_name: Option<&'a str>,
    file_uri: Option<&'a str>,
    custom_types: &'a HashSet<String>,
}

fn parse_variable_statement(stmt: &str, ctx: &VarParseContext, results: &mut Vec<VariableSymbol>) {
    let tokens: Vec<&str> = stmt.split_whitespace().collect();
    if tokens.is_empty() {
        return;
    }

    let mut qualifier = String::new();
    let mut var_type = String::new();
    let mut idents_start_idx = 0;

    for (i, &tok) in tokens.iter().enumerate() {
        let clean_tok = tok.trim_matches([';', ',', '(', ')']);
        if STORAGE_QUALIFIERS.contains(&clean_tok) {
            if qualifier.is_empty() {
                qualifier = clean_tok.to_string();
            } else {
                qualifier.push(' ');
                qualifier.push_str(clean_tok);
            }
        } else if KNOWN_BASE_TYPES.contains(&clean_tok)
            || ctx.custom_types.contains(clean_tok)
            || (!qualifier.is_empty() && var_type.is_empty() && is_valid_identifier(clean_tok))
        {
            var_type = clean_tok.to_string();
            idents_start_idx = i + 1;
            break;
        } else if qualifier.is_empty() && i == 0 {
            if KNOWN_BASE_TYPES.contains(&clean_tok) || ctx.custom_types.contains(clean_tok) {
                var_type = clean_tok.to_string();
                idents_start_idx = 1;
                break;
            } else {
                return;
            }
        }
    }

    if var_type.is_empty() || idents_start_idx >= tokens.len() {
        return;
    }

    if qualifier.is_empty() {
        qualifier = "var".to_string();
    }

    let idents_slice = &tokens[idents_start_idx..];
    let remaining = idents_slice.join(" ");
    let doc_text = if ctx.pending_doc.is_empty() {
        None
    } else {
        Some(ctx.pending_doc.join(" "))
    };

    for part in remaining.split(',') {
        let part_trimmed = part.trim();
        let var_name = part_trimmed
            .split(|c: char| c == '[' || c == ';' || c == '=' || c.is_whitespace())
            .next()
            .unwrap_or("");
        if is_valid_identifier(var_name)
            && !INVALID_NAMES.contains(&var_name)
            && !INVALID_TYPES.contains(&var_name)
        {
            let col = ctx.raw_line.find(var_name).unwrap_or(0);
            results.push(VariableSymbol {
                name: var_name.to_string(),
                var_type: var_type.clone(),
                qualifier: qualifier.clone(),
                doc: doc_text.clone(),
                source: ctx.source_name.map(|s| s.to_string()),
                line: ctx.line_idx,
                col,
                file_uri: ctx.file_uri.map(|s| s.to_string()),
            });
        }
    }
}

struct CacheEntry {
    mtime: SystemTime,
    functions: Vec<FunctionSignature>,
    variables: Vec<VariableSymbol>,
}

static INCLUDE_CACHE: OnceLock<Mutex<HashMap<PathBuf, CacheEntry>>> = OnceLock::new();

fn get_include_cache() -> &'static Mutex<HashMap<PathBuf, CacheEntry>> {
    INCLUDE_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

struct IncludeScanContext<'a> {
    doc_cache: &'a HashMap<String, String>,
    all_functions: Option<&'a mut Vec<FunctionSignature>>,
    all_variables: Option<&'a mut Vec<VariableSymbol>>,
    visited: &'a mut HashSet<PathBuf>,
}

fn scan_included_file(
    candidate: &Path,
    source_label: &str,
    ctx: &mut IncludeScanContext,
    depth: usize,
) {
    if depth > 8 {
        return;
    }

    // 1. If currently open in Zed editor buffer: parse live content (0 disk I/O)
    let clean_path = candidate.to_string_lossy().replace('\\', "/");
    let candidate_uri = format!("file:///{}", clean_path.trim_start_matches('/'));
    if let Some(live_text) = ctx.doc_cache.get(&candidate_uri) {
        if let Some(ref mut funcs_out) = ctx.all_functions {
            let funcs = scan_user_functions(live_text, Some(source_label), Some(&candidate_uri));
            funcs_out.extend(funcs);
        }
        if let Some(ref mut vars_out) = ctx.all_variables {
            let vars = scan_user_variables(live_text, Some(source_label), Some(&candidate_uri));
            vars_out.extend(vars);
        }

        if let Some(parent_dir) = candidate.parent() {
            scan_includes_in_text(live_text, parent_dir, ctx, depth + 1);
        }
        return;
    }

    // 2. If on disk: check mtime cache for instant sub-microsecond response
    if !candidate.is_file() {
        return;
    }

    let mtime = std::fs::metadata(candidate).and_then(|m| m.modified()).ok();

    if let Some(mt) = mtime {
        let cached = {
            if let Ok(cache) = get_include_cache().lock() {
                cache.get(candidate).and_then(|entry| {
                    if entry.mtime == mt {
                        Some((entry.functions.clone(), entry.variables.clone()))
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        };

        if let Some((funcs, vars)) = cached {
            if let Some(ref mut funcs_out) = ctx.all_functions {
                funcs_out.extend(funcs);
            }
            if let Some(ref mut vars_out) = ctx.all_variables {
                vars_out.extend(vars);
            }
            return;
        }

        // Cache miss: read from disk and cache
        if let Ok(disk_text) = std::fs::read_to_string(candidate) {
            let candidate_uri = path_to_uri(candidate);
            let funcs = scan_user_functions(&disk_text, Some(source_label), Some(&candidate_uri));
            let vars = scan_user_variables(&disk_text, Some(source_label), Some(&candidate_uri));
            if let Ok(mut cache) = get_include_cache().lock() {
                if cache.len() > 64 {
                    cache.clear();
                }
                cache.insert(
                    candidate.to_path_buf(),
                    CacheEntry {
                        mtime: mt,
                        functions: funcs.clone(),
                        variables: vars.clone(),
                    },
                );
            }
            if let Some(ref mut funcs_out) = ctx.all_functions {
                funcs_out.extend(funcs);
            }
            if let Some(ref mut vars_out) = ctx.all_variables {
                vars_out.extend(vars);
            }

            if let Some(parent_dir) = candidate.parent() {
                scan_includes_in_text(&disk_text, parent_dir, ctx, depth + 1);
            }
        }
    }
}

fn scan_includes_in_text(text: &str, base_dir: &Path, ctx: &mut IncludeScanContext, depth: usize) {
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("#include") {
            let include_target = rest.trim().trim_matches(['"', '<', '>']);
            if include_target.is_empty() {
                continue;
            }

            let candidate = base_dir.join(include_target);
            if !ctx.visited.insert(candidate.clone()) {
                continue;
            }

            let source_label = candidate
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(include_target);

            scan_included_file(&candidate, source_label, ctx, depth);
        }
    }
}

/// Resolves #include directives and aggregates function signatures with mtime caching.
pub fn resolve_includes_and_scan(
    uri: &str,
    text: &str,
    doc_cache: &HashMap<String, String>,
) -> Vec<FunctionSignature> {
    let mut all_functions = scan_user_functions(text, None, Some(uri));
    let mut visited: HashSet<PathBuf> = HashSet::new();

    if let Some(base_dir) = uri_to_path(uri).and_then(|p| p.parent().map(|dir| dir.to_path_buf())) {
        let mut ctx = IncludeScanContext {
            doc_cache,
            all_functions: Some(&mut all_functions),
            all_variables: None,
            visited: &mut visited,
        };
        scan_includes_in_text(text, &base_dir, &mut ctx, 1);
    }

    all_functions
}

/// Resolves #include directives and aggregates variable and symbol declarations with mtime caching.
pub fn resolve_includes_and_scan_variables(
    uri: &str,
    text: &str,
    doc_cache: &HashMap<String, String>,
) -> Vec<VariableSymbol> {
    let mut all_variables = scan_user_variables(text, None, Some(uri));
    let mut visited: HashSet<PathBuf> = HashSet::new();

    if let Some(base_dir) = uri_to_path(uri).and_then(|p| p.parent().map(|dir| dir.to_path_buf())) {
        let mut ctx = IncludeScanContext {
            doc_cache,
            all_functions: None,
            all_variables: Some(&mut all_variables),
            visited: &mut visited,
        };
        scan_includes_in_text(text, &base_dir, &mut ctx, 1);
    }

    all_variables
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

    if is_in_comment_or_string(doc, line_idx, col_idx) {
        return json!(null);
    }

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
    let matched_funcs: Vec<&FunctionSignature> =
        user_funcs.iter().filter(|f| f.name == fn_name).collect();

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

    if is_in_comment_or_string(doc, line_idx, col_idx) {
        return json!(null);
    }

    let line = match doc.lines().nth(line_idx) {
        Some(l) => l,
        None => return json!(null),
    };

    let safe_col = {
        let max_col = col_idx.min(line.len());
        if line.is_char_boundary(max_col) {
            max_col
        } else {
            (0..=max_col)
                .rev()
                .find(|&i| line.is_char_boundary(i))
                .unwrap_or(0)
        }
    };

    // Extract word under cursor
    let mut word_start = safe_col;
    for (i, c) in line[..safe_col].char_indices().rev() {
        if c.is_alphanumeric() || c == '_' {
            word_start = i;
        } else {
            break;
        }
    }

    let mut word_end = safe_col;
    for (i, c) in line[safe_col..].char_indices() {
        if c.is_alphanumeric() || c == '_' {
            word_end = safe_col + i + c.len_utf8();
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
            func.label, source_info, doc_text
        );

        return json!({
            "contents": {
                "kind": "markdown",
                "value": markdown
            }
        });
    }

    // 3. User-defined variables / symbols
    let user_vars = resolve_includes_and_scan_variables(uri, doc, doc_cache);
    if let Some(var) = user_vars.iter().find(|v| v.name == word) {
        let doc_text = var.doc.as_deref().unwrap_or("");
        let source_info = match &var.source {
            Some(src) => format!("*Defined in `{src}`*\n\n"),
            None => String::new(),
        };

        let markdown = format!(
            "```glsl\n{} {} {}\n```\n\n{}{}",
            var.qualifier, var.var_type, var.name, source_info, doc_text
        );

        return json!({
            "contents": {
                "kind": "markdown",
                "value": markdown.trim_end()
            }
        });
    }

    json!(null)
}

/// Handles textDocument/definition requests.
pub fn handle_definition(msg: &Value, doc_cache: &HashMap<String, String>) -> Value {
    let params = match msg.get("params") {
        Some(p) => p,
        None => return Value::Null,
    };

    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
    let line_idx = params["position"]["line"].as_u64().unwrap_or(0) as usize;
    let col_idx = params["position"]["character"].as_u64().unwrap_or(0) as usize;

    let doc = match doc_cache.get(uri) {
        Some(d) => d,
        None => return Value::Null,
    };

    if is_in_comment_or_string(doc, line_idx, col_idx) {
        return Value::Null;
    }

    let line = match doc.lines().nth(line_idx) {
        Some(l) => l,
        None => return Value::Null,
    };

    let max_col = col_idx.min(line.len());
    let mut word_start = max_col;
    for (i, c) in line[..max_col].char_indices().rev() {
        if c.is_alphanumeric() || c == '_' {
            word_start = i;
        } else {
            break;
        }
    }

    let mut word_end = max_col;
    for (i, c) in line[max_col..].char_indices() {
        if c.is_alphanumeric() || c == '_' {
            word_end = max_col + i + c.len_utf8();
        } else {
            break;
        }
    }

    if word_start >= word_end {
        return Value::Null;
    }
    let word = &line[word_start..word_end];

    let user_funcs = resolve_includes_and_scan(uri, doc, doc_cache);
    if let Some(func) = user_funcs.iter().find(|f| f.name == word) {
        if let Some(target_uri) = &func.file_uri {
            return json!({
                "uri": target_uri,
                "range": {
                    "start": { "line": func.line, "character": func.col },
                    "end": { "line": func.line, "character": func.col + func.name.len() }
                }
            });
        }
    }

    let user_vars = resolve_includes_and_scan_variables(uri, doc, doc_cache);
    if let Some(var) = user_vars.iter().find(|v| v.name == word) {
        if let Some(target_uri) = &var.file_uri {
            return json!({
                "uri": target_uri,
                "range": {
                    "start": { "line": var.line, "character": var.col },
                    "end": { "line": var.line, "character": var.col + var.name.len() }
                }
            });
        }
    }

    Value::Null
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
        let funcs = scan_user_functions(code, None, None);
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
        let funcs = scan_user_functions(code, None, None);
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

    #[test]
    fn test_resolve_includes_live_cache() {
        let mut doc_cache = HashMap::new();
        let common_uri = "file:///project/shaders/common.glsl";
        let common_code = r#"
vec3 getFragPos(mat4 model, vec3 aPos) {
    return vec3(model * vec4(aPos, 1.0));
}
"#;
        doc_cache.insert(common_uri.to_string(), common_code.to_string());

        let main_code = r#"
#include "common.glsl"
void main() {}
"#;
        let funcs =
            resolve_includes_and_scan("file:///project/shaders/main.frag", main_code, &doc_cache);
        assert_eq!(funcs.len(), 2);
        let common_fn = funcs
            .iter()
            .find(|f| f.name == "getFragPos")
            .expect("getFragPos found");
        assert_eq!(common_fn.source, Some("common.glsl".to_string()));
    }

    #[test]
    fn test_is_in_comment_or_string() {
        let code = r#"vec3 a = vec3(1.0); // comment line
/* block comment
   second line */
vec3 b = "string literal";
"#;
        // Line 0: code before '//'
        assert!(!is_in_comment_or_string(code, 0, 10));
        // Line 0: inside comment after '//'
        assert!(is_in_comment_or_string(code, 0, 25));

        // Line 1: inside block comment
        assert!(is_in_comment_or_string(code, 1, 5));
        // Line 2: inside block comment
        assert!(is_in_comment_or_string(code, 2, 5));

        // Line 3: code before quote
        assert!(!is_in_comment_or_string(code, 3, 5));
        // Line 3: inside string
        assert!(is_in_comment_or_string(code, 3, 12));
    }

    #[test]
    fn test_signature_help_in_comment() {
        let mut doc_cache = HashMap::new();
        let uri = "file:///shader.frag";
        let code = "// normalize(vec3(1.0), ";
        doc_cache.insert(uri.to_string(), code.to_string());

        let req = json!({
            "params": {
                "textDocument": { "uri": uri },
                "position": { "line": 0, "character": 15 }
            }
        });
        let res = handle_signature_help(&req, &doc_cache);
        assert!(res.is_null());
    }

    #[test]
    fn test_handle_definition() {
        let mut doc_cache = HashMap::new();
        let uri = "file:///shader.frag";
        let code = "vec3 helper() { return vec3(1.0); }\nvoid main() { helper(); }";
        doc_cache.insert(uri.to_string(), code.to_string());

        let req = json!({
            "params": {
                "textDocument": { "uri": uri },
                "position": { "line": 1, "character": 16 }
            }
        });
        let res = handle_definition(&req, &doc_cache);
        assert_eq!(res["uri"], uri);
        assert_eq!(res["range"]["start"]["line"], 0);
    }

    #[test]
    fn test_scan_user_variables() {
        let code = r#"
#version 460 core
layout(location = 0) in vec3 aPos;
// Fragment position in world space
out vec3 FragPos;
uniform mat4 model, view, projection;
const float PI = 3.14159;
#define NR_LIGHTS 4
struct Material {
    vec3 ambient;
};
void main() {
    vec3 norm = normalize(aPos);
}
"#;
        let vars = scan_user_variables(code, None, Some("file:///shader.frag"));
        assert!(vars
            .iter()
            .any(|v| v.name == "aPos" && v.var_type == "vec3" && v.qualifier == "in"));
        assert!(vars.iter().any(|v| v.name == "FragPos"
            && v.var_type == "vec3"
            && v.qualifier == "out"
            && v.doc.as_ref().unwrap().contains("Fragment position")));
        assert!(vars
            .iter()
            .any(|v| v.name == "model" && v.var_type == "mat4" && v.qualifier == "uniform"));
        assert!(vars
            .iter()
            .any(|v| v.name == "view" && v.var_type == "mat4" && v.qualifier == "uniform"));
        assert!(vars
            .iter()
            .any(|v| v.name == "projection" && v.var_type == "mat4" && v.qualifier == "uniform"));
        assert!(vars
            .iter()
            .any(|v| v.name == "PI" && v.var_type == "float" && v.qualifier == "const"));
        assert!(vars
            .iter()
            .any(|v| v.name == "NR_LIGHTS" && v.qualifier == "#define"));
        assert!(vars
            .iter()
            .any(|v| v.name == "Material" && v.qualifier == "struct"));
        assert!(vars
            .iter()
            .any(|v| v.name == "norm" && v.var_type == "vec3"));
    }

    #[test]
    fn test_variable_hover_and_definition() {
        let mut doc_cache = HashMap::new();
        let uri = "file:///shader.frag";
        let code = "out vec3 FragPos;\nvoid main() { vec3 p = FragPos; }";
        doc_cache.insert(uri.to_string(), code.to_string());

        // Hover test
        let hover_req = json!({
            "params": {
                "textDocument": { "uri": uri },
                "position": { "line": 1, "character": 24 } // hovering on FragPos
            }
        });
        let hover_res = handle_hover(&hover_req, &doc_cache);
        assert!(hover_res["contents"]["value"]
            .as_str()
            .unwrap()
            .contains("out vec3 FragPos"));

        // Definition test
        let def_req = json!({
            "params": {
                "textDocument": { "uri": uri },
                "position": { "line": 1, "character": 24 } // F12 on FragPos
            }
        });
        let def_res = handle_definition(&def_req, &doc_cache);
        assert_eq!(def_res["uri"], uri);
        assert_eq!(def_res["range"]["start"]["line"], 0);
    }
}
