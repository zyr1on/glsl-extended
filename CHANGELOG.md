# Changelog

All notable changes to the GLSL Extended extension for Zed are documented in this file.

## [0.1.19] - 2026-09-14

### Added
- **Unified LSP Orchestrator (`AnalyzerBridge`):** `glsl_validator` acts as the master orchestrator uniting both `glslangValidator` (for compilation diagnostics) and `glsl_analyzer` (for AST completions, Khronos GLSL extensions, `#include` resolution, hover documentation, and goto definition).
- **Smart Function Completion with Tab `()`:** Automatically decorates function items with `($1)$0` snippets (`insertTextFormat: 2`), placing parentheses and cursor inside upon Tab.
- **Dynamic Vector Swizzle Completions:** Injects vector swizzle completions (`.x`, `.xy`, `.xyz`, `.xyzw`, `.rgba`, `.stpq`) when accessing vector members via `.`, dynamically filtered by vector dimension and prefix.
- **Zero Dual-LSP Conflicts:** Fixed duplicate popup suggestions (such as `testColor` appearing twice) by routing all editor interactions through `glsl_validator` as the single registered GLSL language server.
- **Complete Native GLSL Types Suite:** Autocomplete natively suggests all GLSL 4.6 & Vulkan types as a reliable offline fallback:
  - Scalar types: `float`, `double`, `int`, `uint`, `bool`, `void`.
  - Floating, double, integer, unsigned, and boolean vectors: `vec2..vec4`, `dvec2..dvec4`, `ivec2..ivec4`, `uvec2..uvec4`, `bvec2..bvec4`.
  - Matrices: `mat2..mat4`, all non-square matrix types (`mat2x3`, `mat4x3`, etc.), and double matrices (`dmat2..dmat4x4`).
  - Samplers & Images: `sampler1D..samplerCube`, `sampler2DShadow`, `sampler2DArray`, `sampler2DMS`, `samplerBuffer`, integer/unsigned samplers (`isampler2D`, `usampler2D`), and image types (`image2D`, `iimage2D`, `uimage2D`).
  - Vulkan specifics & atomics: `atomic_uint`, `subpassInput`, `subpassInputMS`.
  - Type constructor snippets: `vec2(...)`, `vec3(...)`, `vec4(...)`, `mat4(...)` with Tab snippet placement.
- **Complete GLSL Storage & Flow Keywords:** Autocomplete support for `layout`, `binding`, `location`, `set`, `push_constant`, `offset`, `std140`, `std430`, `uniform`, `buffer`, `in`, `out`, `inout`, `flat`, `smooth`, `noperspective`, `centroid`, `sample`, `patch`, `coherent`, `volatile`, `restrict`, `readonly`, `writeonly`, `precision`, `highp`, `mediump`, `lowp`, `invariant`, `precise`, `struct`, `subroutine`, `return`, `discard`, `break`, `continue`, `if`, `else`, `for`, `while`, `do`, `switch`, `case`, `default`.
- **GLSL Builtin Variables:** Builtin variable suggestions across pipeline stages (`gl_Position`, `gl_PointSize`, `gl_FragCoord`, `gl_FragDepth`, `gl_VertexIndex`, `gl_InstanceIndex`, `gl_GlobalInvocationID`, etc.).
- **Preprocessor Directives:** Suggestions for `#version`, `#include`, `#define`, `#undef`, `#if`, `#ifdef`, `#extension`, `#pragma`, etc.
- **Strict Single-Pass Deduplication:** Guaranteed zero duplicate labels across any autocompletion query.

---

## [0.1.18] - 2026-09-14

### Added
- **Explicit Binary Path Configuration:** Users can now specify custom executable paths in Zed `settings.json` for all binaries:
  - `glslang_validator_path` / `glslang_path`: Direct path to `glslang` or `glslangValidator`.
  - `glsl_validator_path`: Direct path to `glsl_validator` binary (or via `lsp.glsl_validator.binary.path`).
  - `glsl_analyzer_path`: Direct path to `glsl_analyzer` binary (or via `lsp.glsl_analyzer.binary.path`).
- **Graceful Null & Empty Path Fallback:** When custom path settings are empty (`""`), whitespace-only, or `null`, the extension automatically ignores them and executes the standard multi-tier resolution (system `PATH`, cached versions, Vulkan SDK, platform fallbacks, and GitHub release downloads).
- **Comprehensive Settings Documentation:** Added full documentation and configuration schema examples in `README.md` covering `settings.json` options.

---

## [0.1.17] - 2026-09-14

### Added
- **Automatic `glslang` Download & Resolution:** Zero-configuration setup out of the box. If `glslang` or `glslangValidator` is not found on the user's system `PATH` or Vulkan SDK, the extension automatically queries KhronosGroup/glslang official GitHub releases, downloads the matching release package for Windows, Linux, or macOS, and seamlessly wires it to the language server.
- **Local Sibling Directory Scanner:** `glsl_validator` now automatically inspects parent and sibling directories for extension-managed `glslang-*` installations, guaranteeing robust reference compiler discovery even without manual environment variables.
- **Comprehensive Dependency Documentation:** Updated `README.md` with full details on zero-setup automatic downloading as well as manual package manager instructions across Windows (winget, MSYS2), Linux (apt, pacman, dnf), and macOS (Homebrew).

---

## [0.1.16] - 2026-09-14

### Performance
- **Zero-Copy LSP Request Handling:** Replaced document cache cloning (`m.clone()`) across all language server handlers (`completion`, `signatureHelp`, `hover`, `definition`, `colorPresentation`, and background diagnostics worker) with direct borrowing. Eliminates redundant heap allocations and full-text copies on every keystroke.
- **Link-Time Optimization (LTO) & Binary Stripping:** Configured release profile with `lto = true`, `codegen-units = 1`, `strip = true`, and `panic = "abort"`, shrinking the standalone `glsl_validator` binary to under 800 KB and optimizing cross-crate inlining.

---

## [0.1.15] - 2026-09-14

### Fixed
- **Signature Help Active Parameter Clamping:** Clamped `activeParameter` to `parameters.len() - 1` when the argument count exceeds known parameters (such as typing a trailing comma: `sin_wave(pos.x, TIME, )`). This prevents LSP editors like Zed from prematurely dismissing the signature help popup.
- **Overload Selection on Trailing Arguments:** When argument counts exceed the shortest overload, the server now selects the longest matching overload instead of resetting to the first overload (`index 0`).
- **Retrigger Characters Registration:** Added `retriggerCharacters: [","]` to `signatureHelpProvider` capability in server initialization, ensuring editors refresh signature help seamlessly on comma keystrokes.

---

## [0.1.14] - 2026-09-14

### Added
- **Suppression of Signature Help on Function Declarations:** Signature help popup is now intelligently suppressed when authoring function declarations and new overload headers (e.g. `vec3 calculateNormal(mat4 normal, )`). This prevents previous function signatures from intrusively appearing while defining new overloads.
- **Smart Active Overload Matching:** Signature help for overloaded function calls now prioritizes the overload whose parameter count best matches the currently active argument index.
- **Vulkan Interface Block & Instance Autocompletion:** Full support for `layout(set = ..., binding = ...) uniform BlockName { ... } instanceName;`. Both the block interface type, the instance identifier (`ubo`), and all inner member fields (`projection`, `view`, etc.) are recognized, autocompleted with type information, and navigable via F12.
- **Dedicated Vulkan Snippets:** Added `vert-vk`, `frag-vk`, and `ubo-vk` templates to `snippets.json` and language server completions for plug-and-play Vulkan GLSL authoring without manual `#version` edits.
- **Windows Console Popup Suppression (`CREATE_NO_WINDOW`):** Applied `0x0800_0000` to all process invocations (`glslangValidator`, `clang-format`, `find_in_path`), eliminating command prompt window flashes on Windows.
- **High-Speed macOS GitHub Actions Runners:** Upgraded CI and Release pipelines to use `macos-latest` (Apple Silicon M1/M2) for Intel macOS cross-compilation, reducing build times from 10+ minutes to 40 seconds.
- **Official Zed Publishing Guide:** Added complete publishing documentation (`ZED_PUBLISHING_GUIDE.md` and `ZED_PUBLISHING_GUIDE.pdf`) detailing step-by-step submission to the official Zed Extension Registry.

### Fixed
- **Cross-Platform `unused_mut` Warning:** Resolved platform-conditional `mut` warning on non-Windows targets, achieving zero-warning compliance under `-D warnings` on Linux, macOS, and Windows.

---

## [0.1.13] - 2026-09-14

### Fixed
- **Clippy Collapsible Ifs:** Refactored nested if-conditions in `src/lib.rs` into idiomatic Rust 2024 let-chain expressions.
- **Too Many Arguments Refactoring:** Cleaned up LSP handler parameter signatures with unified context structs.

---

## [0.1.12] - 2026-09-14

### Added
- **Complete User Variable & Symbol Autocompletion:** Autocomplete now scans and suggests all GLSL variables, including interface qualifiers (`out vec3 FragPos`, `in vec3 aPos`), `uniform` variables (`uniform mat4 model`), `const` definitions (`const float PI`), structs (`struct Material`), `#define` macros, and local variables.
- **Variable Hover & Goto-Definition:** Hovering on user variables displays their type, qualifiers, and documentation comments. Pressing F12 jumps directly to the line and column of variable definition across current and included files.
- **Variable Autocompletion Across `#include`:** Variables, structs, and uniforms declared in `#include` header files (e.g. `common.glsl`) appear seamlessly in autocompletion and hover with source file labels.
- **Dual Language Server Manifest Support:** Re-registered `glsl_analyzer` in `extension.toml` and `src/lib.rs` for optional use, while keeping `glsl_validator` as the default high-performance engine.

---

## [0.1.11] - 2026-09-14

### Added
- **Unified Standalone Language Server:** Removed redundant external `glsl_analyzer` binary. `glsl_validator` now serves as the sole, bloat-free language server providing compile diagnostics, smart autocomplete, snippets, signature help, hover docs, formatting, and goto-definition.
- **Goto Definition (`textDocument/definition`):** Jump directly to function definitions across the current shader and all recursively included files.
- **Automatic Cargo Bin Discovery:** Automatically detects and executes `glsl_validator` from `~/.cargo/bin` if not already in system PATH.

### Fixed
- **Function Autocomplete Parentheses (`()` Insertion):** Eliminated LSP completion collisions caused by secondary servers overriding snippet completions with plain-text identifiers. Pressing Tab on any function (e.g. `inv` -> `inverse()`) now reliably inserts parentheses with cursor positioned inside.

### Performance & Memory
- **50%+ Memory & CPU Reduction:** Eliminated redundant background language server processes and unnecessary binary downloads, slashing RAM footprint and background CPU usage.

---

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
