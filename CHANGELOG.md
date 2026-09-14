# Changelog

All notable changes to the GLSL Extended extension for Zed are documented in this file.

## [0.1.10] - 2026-09-14

### Added
- **Parent Version Inheritance for Include Files:** Automatically inspects parent shader files (in open editor buffers `doc_cache` and sibling directory files) that `#include` the current file, inheriting their exact `#version` and active `#extension` directives. When editing header files like `common.glsl`, they are now compiled under the exact GLSL profile of the including shader without requiring redundant `#version` tags.
- **Configurable `default_version` Setting:** Users can optionally specify `"default_version"` (e.g. `"330 core"` or `"450 core"`) under `glsl_validator` initialization options or settings for workspace-wide fallback control.
- **Comprehensive GLSL Core Built-ins Database:** Added 25+ missing core GLSL 4.6 functions to `docs.rs` including matrix functions (`transpose`, `inverse`, `determinant`, `matrixCompMult`, `outerProduct`), vector relational functions (`lessThan`, `greaterThan`, `equal`, `notEqual`, `any`, `all`, `not`), fused math (`fma`), bitwise operations (`bitfieldExtract`, `bitCount`, `findLSB`, etc.), and atomics (`atomicAdd`).
- **Precision Snippet Completions with `textEdit`:** Function autocompletions now supply exact character replacement ranges (`textEdit`) and optimized priority ranking (`sortText: "01_..."`), guaranteeing reliable snippet expansion with parentheses and cursor positioning inside `()` across Zed and LSP clients.

---

## [0.1.9] - 2026-09-14

### Added
- **Parentheses Insertion on Function Autocompletion:** Autocompleting functions (e.g. `inverse`, `normalize`, or user-defined functions) now automatically appends `()` snippet with the cursor positioned inside the parentheses (`function($1)$0`). If an opening parenthesis `(` is already present after the cursor, plain text insertion is used to avoid duplicate parentheses.
- **Modern GLSL Header Fallback:** Automatically prepends `#version 460 core` (or `#version 460` for Vulkan) with `#line 1` when validating files that lack an explicit `#version` directive (such as included `.glsl` helper files). This eliminates obsolete GLSL 110 legacy errors (such as `cannot convert from 'const float' to 'matrix'` when calling modern built-ins like `inverse` or `transpose`) while keeping compiler diagnostic line numbers 100% accurate.

### Fixed
- **GLSL 110 Undeclared Function Resolution:** Fixed false-positive compile errors on modern built-in functions (`inverse()`, `transpose()`, etc.) in header/include files that do not declare a `#version` line.

---

## [0.1.8] - 2026-09-14

### Added
- **Intelligent Comment & String Suppression:** Automatically suppresses autocompletion, signature help, and hover tooltips while typing inside single-line comments (`//`), multi-line comments (`/* ... */`), or string literals (`"..."`).
- **Streamlined Documentation:** Refactored and condensed `README.md` for maximum clarity, removing redundant verbose listings while keeping complete quickstart and configuration guides.

### Performance & Security
- **Zero-Allocation Context Scanner:** Added a streaming single-pass cursor context detector (`is_in_comment_or_string`) with zero allocations and cross-platform CRLF/LF support.
- **Defensive Boundary Guards:** Reinforced integer arithmetic and slice indexing against any potential overflow or off-by-one errors.

---

## [0.1.7] - 2026-09-14

### Added
- **Recursive `#include` Function Autocompletion:** Function autocompletion (`textDocument/completion`) now aggregates all functions declared across directly and transitively included files (e.g. `common.glsl`), displaying parameter signatures and file origin in the detail/doc popup.
- **Built-in docs.gl Autocompletion:** 35+ core GLSL mathematical, geometric, and texture functions (`normalize`, `dot`, `cross`, `reflect`, `texture`, etc.) appear in autocompletion with docs.gl summaries.
- **Dynamic OpenGL Version Labeling:** Compiler diagnostic messages now dynamically reflect the `#version` declared in the shader (e.g. `glslangValidator (OpenGL 330 core)` vs `glslangValidator (OpenGL 460 core)` vs `glslangValidator (Vulkan)`).

### Performance & Optimizations
- **mtime File Cache for Includes:** Implemented an mtime-keyed bounded cache (max 64 entries) for disk files. Unchanged include headers skip disk I/O and regex parsing entirely, delivering sub-microsecond responses.
- **Live Memory Priority (Zero Disk I/O):** Open editor buffers in `doc_cache` take precedence over filesystem reads, enabling instant autocompletion of unsaved changes in include files.
- **Zero-Allocation String Scanning:** Replaced intermediate `Vec<&str>` allocations in function header parsing, color token parsing, and autocompletion matching with single-pass iterators and register-based byte comparisons (`starts_with_ignore_ascii_case`).
- **UTF-8 Char Boundary Safety:** Guaranteed crash-free cursor indexing across all LSP hover, completion, and signature queries even with multi-byte Unicode comments.

---

## [0.1.6] - 2026-09-14

### Added
- **Tree-sitter Indentation Correction:** Added closing delimiter matching (`"}" @end`, `")" @end`, `"]" @end`) in `languages/glsl/indents.scm` to prevent extra indentation when pressing Enter between braces.
- **docs.gl Database:** Built-in offline documentation database for GLSL core functions with parameter lists and markdown overviews.
- **Signature Help Provider:** Real-time parameter hints with active parameter highlighting on `(` and `,`.
- **Hover Documentation:** Inline documentation for built-in GLSL functions and user functions across `#include`.

---

## [0.1.5] - 2026-09-13

### Added
- **Multi-File Preprocessing:** `#include` support resolving relative paths, `include/` directories, and parent shader folders.
- **Dual Target API:** Seamless switching between Desktop OpenGL (`-C`) and Vulkan SPIR-V (`-V`).
- **AST Code Formatting:** Hybrid formatting with `clang-format` and pure-Rust fallback engine.
- **Color Picker & Swatches:** Inline color box widgets and picker for `vec3` and `vec4` color literals.
- **Generic Snippets:** Snippets for UBO, SSBO, shaders, structs, and functions.
