# Changelog

All notable changes to the **GLSL Extended** extension for the Zed Editor are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.3] — 2026-09-13

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
