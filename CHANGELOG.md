# Changelog

All notable changes to the GLSL Extended extension for Zed are documented in this file.

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
