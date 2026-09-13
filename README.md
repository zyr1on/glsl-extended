# GLSL Extended — Zed Editor Extension

Comprehensive, high-performance GLSL and shader development extension for the **Zed Editor**, featuring full support for **OpenGL 4.6 (Core Profile)** and **Vulkan (SPIR-V)**.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Zed Extension API](https://img.shields.io/badge/Zed%20Extension%20API-v0.7.0-blue)](https://crates.io/crates/zed_extension_api)
[![CI](https://github.com/zyr1on/zed-glsl-extended/actions/workflows/ci.yml/badge.svg)](https://github.com/zyr1on/zed-glsl-extended/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/zyr1on/zed-glsl-extended?color=green)](https://github.com/zyr1on/zed-glsl-extended/releases)

---

## Features

- **Rich Tree-sitter Syntax Highlighting:**
  - 400+ built-in GLSL types (`vec2` - `dmat4`, samplers, images, atomic counters).
  - Built-in GLSL mathematical, geometric, and texture sampling functions (`texture`, `normalize`, `mix`, `fma`, etc.).
  - Built-in OpenGL shader variables (`gl_Position`, `gl_FragCoord`, `gl_VertexID`, `gl_GlobalInvocationID`, etc.).
  - Robust identifier-based matching (`#match?`) preventing parser crashes on custom keywords like `discard`.

- **Dual Language Server Architecture:**
  - **LSP 1 (`glsl_analyzer`):** Smart autocompletion, inline hover documentation, and goto-definition.
  - **LSP 2 (`glsl_validator`):** Real-time compiler diagnostics and linting powered by `glslangValidator`.

- **Dual Target API Validation (OpenGL 4.6 & Vulkan):**
  - **OpenGL 4.6 (Default):** Validates pure Desktop OpenGL without mandatory SPIR-V layout restrictions (`out vec3 Normal;` compiles cleanly).
  - **Vulkan (SPIR-V):** Strictly enforces SPIR-V layout locations (`layout(location = 0)`), descriptor sets, and push constants.
  - **Zero-Restart Switching:** Toggle dynamically via `settings.json` or per-file `// @target: vulkan` directives.
  - Clear source attribution in diagnostics: `glslangValidator (OpenGL 4.6)` vs `glslangValidator (Vulkan)`.

- **Smart GLSL Vector Swizzling & Chained Member Autocompletion:**
  - Automatic swizzle completion on vectors (`.xyzw`, `.rgba`, `.stpq`) and `.length()`.
  - Works seamlessly on standalone variables (`vec4 a; a.`), struct members (`t.a.`), and chained swizzles (`t.a.xyz.`).
  - Context-aware dimension inference: accurately suggests 2D swizzles for `vec2`, 3D for `vec3`, and 4D for `vec4`.

- **GLSL Generic Boilerplate Snippets:**
  - Clean, generic template skeletons without domain-specific hardcoded names.
  - Available through both native Zed snippets and LSP completion:
    - `ubo`: Generic Uniform Buffer Object (`layout(std140, binding = 0) uniform BlockName { ... };`).
    - `ssbo`: Generic Shader Storage Buffer Object (`layout(std430, binding = 0) buffer BlockName { ... };`).
    - `vert`: Complete OpenGL 4.6 Vertex Shader skeleton.
    - `frag`: Complete OpenGL 4.6 Fragment Shader skeleton.
    - `comp`: Compute Shader template with workgroup size layout.
    - `geom`: Geometry Shader template with primitive processing.
    - `struct`: Generic struct definition.
    - `func`: Generic function declaration with parameter placeholders.
    - `main`: Clean `void main() { ... }` shader entrypoint.

- **Hybrid Code Formatting (`textDocument/formatting`):**
  - Full `textDocument/formatting` support on save (`format_on_save`) or on demand (`editor: format`).
  - **Clang-Format Engine:** Uses `clang-format` if detected on `PATH`, `CLANG_FORMAT_PATH`, or Windows MSYS2 UCRT64.
  - **Zero-Dependency Built-in Fallback:** Automatically active when `clang-format` is not installed or fails. Seamlessly indents braces, handles `#version` / `#include` preprocessor column alignment, normalizes blank lines, and cleans whitespace in pure Rust.

- **Interactive Document Color Swatches & Picker:**
  - Real-time inline color swatch previews for `vec3(...)` and `vec4(...)` color literals in shaders.
  - Interactive color picker support via `textDocument/colorPresentation` to visually choose colors and format them back into shaders.

- **Relative `#include` Directory Resolution:**
  - Automatic include path discovery (`-I<parent_dir>`, `-I<parent_dir>/include`, `-I<parent_dir>/shaders`).
  - Allows seamless multi-file shader projects using `#include "common.glsl"` without compiler errors.

- **Cross-Platform Compatibility:**
  - Fully compatible with **Windows**, **Linux**, and **macOS** (both Apple Silicon and Intel).

---

## Supported Shader Stages & Extensions

| Stage | File Extensions |
|---|---|
| **Vertex Shader** | `.vert` |
| **Fragment Shader** | `.frag` |
| **Geometry Shader** | `.geom` |
| **Tessellation Control & Eval** | `.tesc`, `.tese` |
| **Compute Shader** | `.comp` |
| **Mesh & Task Shaders** | `.mesh`, `.task` |
| **Ray Tracing Pipelines** | `.rgen`, `.rint`, `.rahit`, `.rchit`, `.rmiss`, `.rcall` |
| **Generic GLSL / Headers** | `.glsl`, `.glslh` |
| **Header Matching** | Any file starting with `#version \d+` is automatically recognized |

---

## Requirements & External Tools

This extension coordinates two core external tools to provide a complete IDE experience:

### 1. `glsl_analyzer` (Autocomplete, Hover, Goto Definition)
- **Automatic:** Zed downloads `glsl_analyzer` automatically on first launch if it is not already found in your system `PATH`.
- **Manual Download (Optional):** [nolanderc/glsl_analyzer Releases](https://github.com/nolanderc/glsl_analyzer/releases).

---

### 2. `glslangValidator` (Compiler Diagnostics & Linting)
`glslangValidator` is the official reference compiler for GLSL maintained by the [Khronos Group](https://github.com/KhronosGroup/glslang). It powers the real-time diagnostic squiggly lines in Zed.

> **Note:** If `glslangValidator` is not found on your system, Zed will show a friendly warning on line 1 reminding you to install it. Autocompletion and syntax highlighting will continue to function normally.

#### How to Install `glslangValidator`:

- **Windows:**
  - **Option A (Recommended — Vulkan SDK):** Download from [LunarG Vulkan SDK](https://vulkan.lunarg.com/sdk/home) or via winget:
    ```powershell
    winget install KhronosGroup.VulkanSDK
    ```
  - **Option B (Khronos Official Release):** Download pre-built standalone binaries from [KhronosGroup/glslang Releases](https://github.com/KhronosGroup/glslang/releases) (the archive contains `bin/glslangValidator.exe`). Extract and add `bin/` to your system `PATH`.
  - **Option C (MSYS2 / UCRT64):**
    ```bash
    pacman -S mingw-w64-ucrt-x86_64-glslang
    ```

- **Linux:**
  - **Debian / Ubuntu:**
    ```bash
    sudo apt update && sudo apt install glslang-tools
    ```
  - **Arch Linux:**
    ```bash
    sudo pacman -S glslang
    ```
  - **Fedora:**
    ```bash
    sudo dnf install glslang
    ```

- **macOS:**
  - **Homebrew:**
    ```bash
    brew install glslang
    ```

---

### 3. `glsl_validator` (LSP Bridge)
- **Automatic:** Zed downloads the pre-built `glsl_validator` binary for your OS and architecture automatically from [zyr1on/zed-glsl-extended Releases](https://github.com/zyr1on/zed-glsl-extended/releases).
- **Manual Build (Optional):**
  ```bash
  cargo install --path glsl_validator
  ```

---

## Installation

### Installing as a Dev Extension in Zed

1. Clone this repository:
   ```bash
   git clone https://github.com/zyr1on/zed-glsl-extended.git
   ```
2. Open **Zed**.
3. Open the Command Palette (`Ctrl+Shift+P` on Windows/Linux, `Cmd+Shift+P` on macOS).
4. Type and select: **`zed: install dev extension`**.
5. Choose the cloned `zed-glsl-extended` directory.
6. Zed will compile the WebAssembly component and activate the extension immediately.

### Configuration (`settings.json`)

Add or verify the following configuration in your Zed settings (`Ctrl+,` or `Ctrl+Shift+P` -> `zed: open settings`):

```json
{
    "languages": {
        "GLSL": {
            "language_servers": ["glsl_analyzer", "glsl_validator"],
            "tab_size": 4,
            "format_on_save": "off"
        }
    }
}
```

#### Switching Validation Target (OpenGL 4.6 vs. Vulkan)

By default, `glsl_validator` validates against **pure Desktop OpenGL 4.6** (`-C`), allowing standard declarations like `out vec3 Normal;` without mandatory SPIR-V layout locations.

If you are developing for **Vulkan**, you can configure the target API in two ways:

##### Method A: In `settings.json` (Global or Workspace `.zed/settings.json`)

Open Command Palette (`Ctrl+Shift+P`), choose **`zed: open settings`** (or **`zed: open local settings`** for project-specific settings) and set:

```json
{
    "lsp": {
        "glsl_validator": {
            "initialization_options": {
                "target_api": "vulkan" // "opengl" (default) or "vulkan"
            }
        }
    }
}
```

##### Method B: Per-File Inline Directive (Quick Toggle)

You can also override the target API on a per-file basis without touching your settings! Simply place a directive in the first few lines of your shader:

```glsl
// @target: vulkan
#version 460 core

layout(location = 0) in vec3 inPosition;
layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(inPosition, 1.0);
}
```

Or for OpenGL:
```glsl
// @target: opengl
#version 460 core

out vec4 outColor; // Completely valid in OpenGL mode!
```

---

#### 2. Code Formatting Configuration (Built-in vs. Clang-Format)

`glsl_extended` provides dual-engine formatting with full support for both whole-document (`editor: format`) and range/selection formatting (`editor: format_selections` / `Ctrl + K, Ctrl + F`):

- **Built-in Pure-Rust Engine (Default):** Zero external dependencies required! Formats instantly on any OS. Handles brace indentation, preprocessor alignment (`#version`, `#include` at column 0), comma spacing (`vec3(1.0, 2.0)`), and blank line normalization.
- **Clang-Format Engine:** Uses system `clang-format` if available, with automatic fallback to the built-in formatter.

##### Configuring Formatter in Zed `settings.json`:

Add the following to your Zed `settings.json` (Command Palette → `zed: open settings`):

```json
{
  "languages": {
    "GLSL": {
      "format_on_save": "on",
      "formatter": {
        "language_server": {
          "name": "glsl_validator"
        }
      }
    }
  },
  "lsp": {
    "glsl_validator": {
      "initialization_options": {
        "target_api": "opengl",     // "opengl" (default) or "vulkan"
        "formatter": "builtin"      // "builtin" (default) or "clang-format"
      }
    }
  }
}
```

##### Per-File Formatter Directive:
You can switch the formatter per shader file using an inline directive in the first few lines:
```glsl
// @formatter: clang-format
#version 460 core
...
```
Or force the built-in formatter:
```glsl
// @formatter: builtin
#version 460 core
...
```

##### Supported Keybindings & Commands:
- **Format Document:** Command Palette → `editor: format` (or `Shift + Alt + F`).
- **Format Selection:** Select text → `editor: format_selections` (or `Ctrl + K, Ctrl + F`).
- **Format on Save:** Automatically formats on `Ctrl + S` when `"format_on_save": "on"` is set.

---

## Repository Structure

```
zed-glsl-extended/
├── .github/
│   └── workflows/
│       └── release.yml          # Automated multi-platform binary release
├── glsl_validator/              # Linter LSP bridge source code
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── languages/
│   └── glsl/
│       ├── brackets.scm         # Bracket matching queries
│       ├── config.toml          # Language metadata and suffixes
│       ├── highlights.scm       # Tree-sitter syntax highlighting
│       ├── indents.scm          # Auto-indentation queries
│       └── outline.scm          # Code outline symbol queries
├── src/
│   └── lib.rs                   # Zed Extension WASM entry point
├── Cargo.toml                   # Root package manifest
├── extension.toml               # Zed Extension manifest
├── LICENSE                      # MIT License
├── .gitignore                   # Git ignore patterns
└── README.md                    # Documentation
```

---

## Troubleshooting

- **Zed logs:** Open Command Palette → `zed: open log`.
- **Diagnostics bridge logs:** `%LOCALAPPDATA%\Temp\glsl_validator.log` (Windows) or `/tmp/glsl_validator.log` (Linux/macOS).

---

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for detailed release notes and migration guides across all versions.

---

## Author

- **Semih Özdemir** ([@zyr1on](https://github.com/zyr1on)) — `semihozdmirr@gmail.com`

---

## License

This project is licensed under the [MIT License](LICENSE).