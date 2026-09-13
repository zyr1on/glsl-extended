# GLSL Extended for Zed

Comprehensive GLSL and shader development extension for the Zed Editor, featuring full support for OpenGL 4.6 (Core Profile) and Vulkan (SPIR-V) validation, AST-based formatting, smart vector swizzling, snippets, and Tree-sitter syntax highlighting.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Zed Extension API](https://img.shields.io/badge/Zed%20Extension%20API-v0.7.0-blue)](https://crates.io/crates/zed_extension_api)
[![Release](https://img.shields.io/github/v/release/zyr1on/zed-glsl-extended?color=green)](https://github.com/zyr1on/zed-glsl-extended/releases)

---

## Features

- **Rich Tree-sitter Syntax Highlighting:**
  - 400+ built-in GLSL types (`vec2` through `dmat4`, samplers, images, atomic counters).
  - Built-in GLSL mathematical, geometric, and texture sampling functions (`texture`, `normalize`, `mix`, `fma`, etc.).
  - Built-in OpenGL shader variables (`gl_Position`, `gl_FragCoord`, `gl_VertexID`, `gl_GlobalInvocationID`, etc.).
  - Identifier-based matching preventing parser crashes on custom keywords.

- **Dual Language Server Architecture:**
  - **LSP 1 (`glsl_analyzer`):** Fast autocompletion, inline hover documentation, and goto-definition.
  - **LSP 2 (`glsl_validator`):** Asynchronous compiler diagnostics and linting powered by `glslangValidator`.

- **Dual Target API Validation (OpenGL 4.6 & Vulkan):**
  - **OpenGL 4.6 (Default):** Validates pure Desktop OpenGL without mandatory SPIR-V layout restrictions (`out vec3 Normal;` compiles cleanly).
  - **Vulkan (SPIR-V):** Strictly enforces SPIR-V layout locations (`layout(location = 0)`), descriptor sets, and push constants.
  - **Zero-Restart Switching:** Toggle dynamically via `settings.json` or per-file `// @target: vulkan` directives.
  - Source attribution in diagnostics: `glslangValidator (OpenGL 4.6)` vs `glslangValidator (Vulkan)`.

- **Smart GLSL Vector Swizzling & Chained Member Autocompletion:**
  - Automatic swizzle completion on vectors (`.xyzw`, `.rgba`, `.stpq`) and `.length()`.
  - Works on standalone variables (`vec4 a; a.`), struct members (`t.a.`), and chained swizzles (`t.a.xyz.`).
  - Context-aware dimension inference: suggests 2D swizzles for `vec2`, 3D for `vec3`, and 4D for `vec4`.

- **GLSL Generic Boilerplate Snippets:**
  - Clean template skeletons without domain-specific hardcoded names.
  - Available through both native Zed snippets and LSP completion:
    - `ubo`: Uniform Buffer Object (`layout(std140, binding = 0) uniform BlockName { ... };`).
    - `ssbo`: Shader Storage Buffer Object (`layout(std430, binding = 0) buffer BlockName { ... };`).
    - `vert`: Complete OpenGL 4.6 Vertex Shader skeleton.
    - `frag`: Complete OpenGL 4.6 Fragment Shader skeleton.
    - `comp`: Compute Shader template with workgroup size layout.
    - `geom`: Geometry Shader template with primitive processing.
    - `struct`: Generic struct definition.
    - `func`: Generic function declaration with parameter placeholders.
    - `main`: Clean `void main() { ... }` shader entrypoint.

- **Hybrid Code Formatting (`textDocument/formatting`):**
  - **Clang-Format Engine (Default):** Uses system `clang-format` for AST-level formatting. Merges multi-line assignments, cleans redundant blank lines and spaces, indents struct definitions and brace-less `if` statements.
  - **Built-in Pure-Rust Engine (Automatic Fallback):** Activates automatically if `clang-format` is not installed or fails. Handles brace indentation, preprocessor column-0 alignment, comma spacing, and blank line normalization.

- **Multi-File `#include` Resolution:**
  - Two-stage preprocessor resolves `#include` directives across shader folders, `include/` subdirectories, and parent directories without compiler errors.

- **Interactive Document Color Swatches & Picker:**
  - Inline color previews for `vec3(...)` and `vec4(...)` color literals.
  - Interactive color picker support via `textDocument/colorPresentation` to choose colors visually.

---

## Supported Shader Stages & File Extensions

| Stage | File Extensions |
|---|---|
| Vertex Shader | `.vert` |
| Fragment Shader | `.frag` |
| Geometry Shader | `.geom` |
| Tessellation Control & Eval | `.tesc`, `.tese` |
| Compute Shader | `.comp` |
| Mesh & Task Shaders | `.mesh`, `.task` |
| Ray Tracing Pipelines | `.rgen`, `.rint`, `.rahit`, `.rchit`, `.rmiss`, `.rcall` |
| Generic GLSL / Headers | `.glsl`, `.glslh` |
| Header Matching | Any file starting with `#version \d+` is automatically recognized |

---

## Requirements & External Tools

This extension coordinates external tools to provide a complete IDE experience:

### 1. `glslangValidator` (Compiler Diagnostics & Linting)

`glslangValidator` is the official reference compiler for GLSL maintained by the Khronos Group. It powers real-time diagnostic squiggly lines in Zed.

> **Important:** `glslangValidator` must be installed on your system and accessible via your system `PATH` (or located at standard SDK paths like `VULKAN_SDK/bin` or `C:\msys64\ucrt64\bin`).

#### How to Install `glslangValidator`:

- **Windows:**
  - **Option A (Vulkan SDK - Recommended):** Install via [LunarG Vulkan SDK](https://vulkan.lunarg.com/sdk/home) or winget:
    ```powershell
    winget install KhronosGroup.VulkanSDK
    ```
  - **Option B (Khronos Official Release):** Download pre-built standalone binaries from [KhronosGroup/glslang Releases](https://github.com/KhronosGroup/glslang/releases) (extract `bin/glslangValidator.exe` and add to your system `PATH`).
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

### 2. `glsl_analyzer` (Autocomplete, Hover, Navigation)

- **Automatic:** Zed downloads `glsl_analyzer` automatically into its extension directory on first launch if it is not found on your system `PATH`.
- **Manual Download (Optional):** Pre-built releases are available at [nolanderc/glsl_analyzer Releases](https://github.com/nolanderc/glsl_analyzer/releases).

---

### 3. `clang-format` (Optional, for AST Formatting)

- If installed on your system `PATH` (or `C:\msys64\ucrt64\bin\clang-format.exe`), `glsl_validator` will use it by default for AST-level code formatting.
- If not installed, formatting automatically falls back to the built-in pure-Rust formatter with zero setup required.
- To install on Windows:
  ```bash
  pacman -S mingw-w64-ucrt-x86_64-clang-tools-extra
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
6. Zed will activate the extension immediately.

---

## Configuration (`settings.json`)

To configure GLSL settings, open your Zed settings (`Ctrl+,` or Command Palette -> `zed: open settings`):

```json
{
  "languages": {
    "GLSL": {
      "tab_size": 4,
      "format_on_save": "off",
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
        "target_api": "opengl",
        "formatter": "clang-format"
      }
    }
  }
}
```

### Configuration Options & Alternatives

#### 1. Validation Target (`target_api`)
Located under `lsp.glsl_validator.initialization_options.target_api`.

- `"opengl"` (Default):
  Validates against Desktop OpenGL 4.6 (`-C`). Standard declarations like `out vec3 Normal;` compile cleanly without requiring SPIR-V layout locations.
- `"vulkan"`:
  Validates against Vulkan SPIR-V (`-V`). Strictly checks descriptor sets, push constants, and requires explicit input/output locations (`layout(location = 0)`).

#### 2. Formatter Engine (`formatter`)
Located under `lsp.glsl_validator.initialization_options.formatter`.

- `"clang-format"` (Default):
  Uses `clang-format` from your system for AST-level formatting. Reassembles split statements, indents struct definitions, cleans irregular spacing, and indents single-line `if` statements. Automatically falls back to the built-in engine if `clang-format` is unavailable.
- `"builtin"`:
  Forces the internal pure-Rust formatter. Operates with zero external dependencies, handling basic brace indentation and preprocessor alignment.

#### 3. Format on Save (`format_on_save`)
Located under `languages.GLSL.format_on_save`.

- `"off"` (Recommended default):
  Code is only formatted when explicitly requested via keyboard shortcut (`Ctrl + K, Ctrl + F`), preventing unwanted automatic changes while typing or saving.
- `"on"`:
  Automatically formats the document every time the file is saved (`Ctrl + S`).

#### 4. Tab Size (`tab_size`)
Located under `languages.GLSL.tab_size`.

- `4` (Default): Uses 4 spaces per indentation level.
- `2`: Uses 2 spaces per indentation level.
- `8`: Uses 8 spaces per indentation level.

---

### Per-File Inline Directives

You can override validation target and formatter engine per shader file without changing your global settings. Add directives within the first few lines of your shader:

```glsl
// @target: vulkan
// @formatter: clang-format
#version 460 core

layout(location = 0) in vec3 inPosition;
layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(inPosition, 1.0);
}
```

Available directives:
- `// @target: opengl` - Force Desktop OpenGL 4.6 validation.
- `// @target: vulkan` - Force Vulkan SPIR-V validation.
- `// @formatter: clang-format` - Use clang-format engine.
- `// @formatter: builtin` - Force built-in pure-Rust engine.

---

## Keybindings & Commands

- **Format Document:** Command Palette -> `editor: format` (or `Shift + Alt + F`).
- **Format Selection:** Select code -> `editor: format_selections` (or `Ctrl + K, Ctrl + F`).
- **Save without Formatting:** Default behavior when `"format_on_save": "off"`.

---

## Repository Structure

```
zed-glsl-extended/
├── .github/
│   └── workflows/
│       └── release.yml          # Automated multi-platform binary release
├── glsl_validator/              # Diagnostics & formatting LSP bridge source code
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

## Author

- **Semih Özdemir** ([@zyr1on](https://github.com/zyr1on)) - `semihozdmirr@gmail.com`

---

## License

This project is licensed under the [MIT License](LICENSE).
