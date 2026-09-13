# Changelog

All notable changes to the **GLSL Extended** extension for the Zed Editor are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.5] — 2026-09-14

### Added
- **Cross-Platform Binary Discovery Hierarchy:**
  - Prioritizes system `PATH` lookup universally across Windows, Linux, and macOS without process-spawning overhead.
  - Added support for explicit custom binary paths in `settings.json` (`clang_format_path` and `glslang_validator_path`).
  - Added OS-specific fallback directories for official LLVM, Vulkan SDK, MSYS2, `/usr/bin`, and Homebrew.
  - Informative one-time diagnostic log when `clang-format` is missing, showing package manager install commands (`winget`, `pacman`, `apt`, `dnf`, `brew`).
- **Non-Blocking Background Worker Thread with Queue Draining:**
  - Background thread validates shaders via `mpsc` channel, draining outdated rapid-keystroke requests (`rx.try_recv()`).
  - Completely decouples validation from the main LSP event loop, ensuring zero autocomplete latency.
- **Desktop OpenGL `#include` Preprocessor Pipeline:**
  - Added automatic preprocessing pass (`-E -S <stage>`) resolving relative `#include` headers before validation.
- **Universal `extension.wasm` Release Packaging:**
  - Added automatic WebAssembly build and packaging to GitHub Actions release pipeline.
- **Comprehensive Documentation:**
  - Completely overhauled `README.md` with detailed tool installation guides, binary discovery hierarchy, and `settings.json` path configuration examples.
  - 16/16 unit tests passing with zero Clippy warnings.

---

## [0.1.4] — 2026-09-13

### Added
- **GLSL Generic Boilerplate Snippets:**
  - Added clean, generic code snippets for both Zed native snippets (`languages/glsl/snippets.json`) and LSP completion (`textDocument/completion`):
    - `ubo`: Clean Uniform Buffer Object block skeleton with generic placeholders (`layout(std140, binding = 0) uniform BlockName { ... };`).
    - `ssbo`: Clean Shader Storage Buffer Object block skeleton (`layout(std430, binding = 0) buffer BlockName { ... };`).
    - `vert`: Complete OpenGL 4.6 Vertex Shader skeleton (`#version 460 core`, in position, `gl_Position`).
    - `frag`: Complete OpenGL 4.6 Fragment Shader skeleton (`#version 460 core`, out fragColor, `gl_FragColor`).
    - `comp`: Complete Compute Shader skeleton (`layout(local_size_x = 16, ...) in`).
    - `geom`: Complete Geometry Shader skeleton (`triangles` in, `triangle_strip` out, primitive emission loop).
    - `struct`: Generic struct declaration template.
    - `func`: Generic function signature template.
    - `main`: Clean `void main() { ... }` entrypoint template.
- **Hybrid Code Formatting & Range Formatting (`textDocument/formatting` & `textDocument/rangeFormatting`):**
  - Advertised both `"documentFormattingProvider": true` and `"documentRangeFormattingProvider": true` in `glsl_validator`.
  - Supports both full document formatting (`editor: format`) and selection/range formatting (`editor: format_selections` / `Ctrl + K, Ctrl + F`).
  - **Built-in Pure Rust Engine (Default):** Zero dependencies! Formats immediately out of the box with brace indentation, `#version` / `#include` preprocessor column-0 alignment, comma spacing (`vec3(1.0, 2.0)`), and blank line normalization.
  - **Clang-Format Engine:** Optional AST-level formatting using `clang-format`, configurable via `settings.json` or per-file `// @formatter: clang-format` directive.
  - Reordered language servers in `config.toml` to prioritize `glsl_validator` over `glsl_analyzer` for formatting.
- **Document Color Provider (`textDocument/documentColor` & `textDocument/colorPresentation`):**
  - Real-time inline color swatch previews for `vec3(...)` and `vec4(...)` color constructors in GLSL shaders.
  - Interactive color picker support via `textDocument/colorPresentation`, allowing users to adjust colors visually and write back formatted `vec3`/`vec4` literals.
- **Automatic Relative `#include` Directory Resolution:**
  - `glsl_validator` automatically resolves the active shader file's directory and passes `-I<parent_dir>`, `-I<parent_dir>/include`, and `-I<parent_dir>/shaders` to `glslangValidator`.
  - Fixes missing `#include` resolution errors when editing modular shader projects with local header files.
- **Expanded Unit Test Suite:**
  - Added comprehensive tests for snippet generation, color token parsing, color extraction, URI path percent-decoding, syntax cleaning, and FormatterEngine detection (14 unit tests passing).

---

## [0.1.3] — 2026-09-13

### Added
- **Smart GLSL Vector Swizzling & Chained Member Autocompletion:**
  - Integrated LSP `textDocument/completion` into `glsl_validator` triggered on `.`.
  - Intelligently infers vector dimension (2, 3, or 4) from document variable declarations, struct members (`t.a.`), built-in variables, and chained swizzles (`t.a.xyz.`).
  - Provides coordinate swizzles (`.xyzw`), color swizzles (`.rgba`), texture swizzles (`.stpq`), and `.length()` method completions with high-priority sorting.

### Fixed
- **`glsl_analyzer` Nested `bin/` Directory Resolution:**
  - Resolved `failed to spawn command (os error 2)` caused by `glsl_analyzer` release archives extracting the executable inside a `bin/` subfolder (`glsl_analyzer-vX/bin/glsl_analyzer.exe`).
  - Added robust candidate resolution that checks both `{version_dir}/bin/glsl_analyzer{exe}` and `{version_dir}/glsl_analyzer{exe}`.
- **Linux Musl Release Asset Naming:**
  - Corrected asset matching on Linux to use `x86_64-linux-musl.zip` and `aarch64-linux-musl.zip`.
- **`glsl_validator` Path Fallback:**
  - Added dual candidate checks for `{version_dir}/glsl_validator` and `{version_dir}/bin/glsl_validator`.

---

## [0.1.2] — 2026-09-13

### Added
- **Dual Target API Validation (OpenGL 4.6 Core vs. Vulkan):**
  - Configurable validation mode allowing developers to choose between pure Desktop OpenGL 4.6 (`-C`) and Vulkan SPIR-V (`-V`).
  - **Zero-restart Dynamic Re-validation:** Changing `target_api` in Zed settings immediately re-validates all open documents via `workspace/didChangeConfiguration`.
- **Per-File Inline Directives:**
  - Added `// @target: vulkan` and `// @target: opengl` (also supports `/* @target: ... */` and `#pragma target(...)`) in the first 10 lines of any shader to override validation on a per-file basis without touching global settings.
- **Informative Diagnostic Source Labels:**
  - Diagnostic messages now clearly identify the active API: `glslangValidator (OpenGL 4.6)` or `glslangValidator (Vulkan)`.
- **Automated Unit Tests:**
  - Added test coverage for target API string parsing and inline directive detection in `glsl_validator`.

---

## [0.1.1] — 2026-09-13

### Added
- **Missing Compiler Warning:**
  - When `glslangValidator` or `glslang` is not found in `PATH` or `VULKAN_SDK`, `glsl_validator` produces an informative line-0 warning diagnostic with installation guidance instead of failing silently.
- **Continuous Integration (CI):**
  - Added `.github/workflows/ci.yml` running `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` across the extension WASM and the `glsl_validator` daemon on every commit and pull request.
- **Documentation Badges:**
  - Added CI build status badge to `README.md`.

### Fixed
- Collapsed nested `if let` blocks in `src/lib.rs` to satisfy Clippy standards.

---

## [0.1.0] — 2026-09-13

### Initial Release
- **Rich Tree-sitter Syntax Highlighting:**
  - 400+ built-in GLSL types (`vec2`-`dmat4`, samplers, images, atomic counters).
  - Built-in GLSL functions, variables (`gl_Position`, `gl_FragCoord`, etc.).
  - Robust `#match?` identifier query handling preventing parser crashes on keywords like `discard`.
- **Dual Language Server Architecture:**
  - `glsl_analyzer` LSP for intelligent autocomplete, hover documentation, and goto-definition.
  - `glsl_validator` LSP bridge wrapping `glslangValidator` for real-time compile errors and squiggly underlines.
- **Cross-Platform Automated Release Pipeline:**
  - Multi-platform GitHub Actions building native binaries for Windows x86_64, Linux x86_64, macOS aarch64 (Apple Silicon), and macOS x86_64.
  - Automatic download and caching of binaries inside Zed's extension workspace directory.
