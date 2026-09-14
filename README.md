# GLSL Extended for Zed

Comprehensive GLSL and shader development extension for the Zed Editor, featuring full support for Desktop OpenGL (from `#version 330 core` through `#version 460 core`) and Vulkan (SPIR-V) validation, AST-based formatting, smart vector swizzling, recursive `#include` autocompletion, signature help with docs.gl, and Tree-sitter syntax highlighting.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Zed Extension API](https://img.shields.io/badge/Zed%20Extension%20API-v0.7.0-blue)](https://crates.io/crates/zed_extension_api)
[![Release](https://img.shields.io/github/v/release/zyr1on/zed-glsl-extended?color=green)](https://github.com/zyr1on/zed-glsl-extended/releases)

---

https://github.com/user-attachments/assets/51fef2aa-2fce-43d7-9918-d6d61b49b02c

---

> [!IMPORTANT]
> ### Required External Dependency: `glslangValidator`
> Real-time compile diagnostics, error squiggles, and linting require **`glslangValidator`** (the official Khronos Group GLSL reference compiler) to be installed on your system.
> 
> - **Pre-built Standalone Binaries (All Platforms):**
>   You can download official ready-to-run binaries directly from GitHub Releases:
>   [https://github.com/KhronosGroup/glslang/releases](https://github.com/KhronosGroup/glslang/releases)
>   *(Download the archive for your OS, extract `glslangValidator` / `glslangValidator.exe`, and add its folder to your system `PATH`).*
> 
> - **Windows Package Managers:**
>   - **winget / Vulkan SDK:** `winget install KhronosGroup.VulkanSDK` (or via [LunarG Vulkan SDK](https://vulkan.lunarg.com/sdk/home))
>   - **MSYS2 (UCRT64 / MINGW64):** `pacman -S mingw-w64-ucrt-x86_64-glslang`
> 
> - **Linux Package Managers:**
>   - Ubuntu / Debian: `sudo apt install glslang-tools`
>   - Arch Linux: `sudo pacman -S glslang`
>   - Fedora: `sudo dnf install glslang`
> 
> - **macOS (Homebrew):**
>   `brew install glslang`

---

## Features

- **Dual Language Server Architecture:**
  - **LSP 1 (`glsl_analyzer`):** High-speed autocompletion, hover documentation, and goto-definition.
  - **LSP 2 (`glsl_validator`):** Compiler diagnostics, hybrid code formatting, color preview, and signature help.

- **Signature Help & docs.gl Documentation (`signatureHelp`, `hover`):**
  - Real-time parameter hints with active argument highlighting while typing inside function calls.
  - Built-in OpenGL 4.6 documentation (`texture`, `normalize`, `mix`, `clamp`, `dot`, `cross`, etc.) with docs.gl summaries.
  - Resolves user-defined functions across `#include` files using live editor buffer caching (zero disk I/O) and filesystem mtime caching.

- **Recursive `#include` Function Autocompletion:**
  - Autocomplete functions declared across directly and transitively included files (`common.glsl`, etc.).
  - Intelligent context scanner suppresses autocomplete, signature help, and hover popups inside comments (`//`, `/* */`) and string literals.

- **Dual Target API Validation (Desktop OpenGL & Vulkan):**
  - **Desktop OpenGL (Default):** Validates pure Desktop OpenGL without mandatory SPIR-V layout restrictions across all versions (from `#version 330 core` through `#version 460 core`).
  - **Vulkan (SPIR-V):** Strictly enforces SPIR-V layout locations (`layout(location = 0)`), descriptor sets, and push constants.
  - **Dynamic Diagnostics Source:** Diagnostic messages dynamically match the shader version declared in the file (e.g. `glslangValidator (OpenGL 330 core)` vs `glslangValidator (Vulkan)`).
  - **Zero-Restart Switching:** Toggle dynamically via `settings.json` or per-file inline directives.

- **Smart Vector Swizzling & Generic Snippets:**
  - Dimension-aware swizzle completions on vectors (`.xyzw`, `.rgba`, `.stpq`) and `.length()`.
  - Generic boilerplate snippets (`ubo`, `ssbo`, `vert`, `frag`, `comp`, `geom`, `struct`, `func`, `main`).

- **Hybrid Code Formatting (`textDocument/formatting`):**
  - **Clang-Format Engine:** Uses system `clang-format` for AST-level formatting.
  - **Built-in Pure-Rust Fallback:** Automatically activates if `clang-format` is not installed on the system.

- **Interactive Color Swatches & Picker:**
  - Inline color previews for `vec3(...)` and `vec4(...)` color literals with interactive color picker support.

- **Tree-sitter Syntax Highlighting:**
  - 400+ built-in GLSL types, mathematical/geometric functions, built-in variables (`gl_Position`, `gl_FragCoord`, etc.), and proper brace indentation matching.

---

## Configuration (`settings.json`)

To configure GLSL settings in Zed, open your settings (`Ctrl+,` on Windows/Linux, `Cmd+,` on macOS):

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
        "formatter": "clang-format",
        "glslang_validator_path": "",
        "clang_format_path": ""
      }
    }
  }
}
```

### Configuration Options Breakdown

#### `languages.GLSL` Section
- **`tab_size`** *(integer, default: `4`)*: Number of spaces for indentation (`2`, `4`, or `8`).
- **`format_on_save`** *(string, default: `"off"`)*:
  - `"off"`: Formatting only runs when manually triggered (`Shift + Alt + F` or Command Palette -> `editor: format`).
  - `"on"`: Automatically formats the document every time the file is saved (`Ctrl + S`).
- **`formatter`**:
  - `{"language_server": {"name": "glsl_validator"}}`: Directs formatting requests to the GLSL validator language server.

#### `lsp.glsl_validator.initialization_options` Section
- **`target_api`** *(string, default: `"opengl"`)*:
  - `"opengl"`: Validates against Desktop OpenGL rules. Automatically respects the `#version` directive in the shader (e.g. `#version 330 core`, `#version 460 core`, `#version 300 es`). Standard output declarations like `out vec3 Normal;` compile cleanly without requiring explicit layout locations.
  - `"vulkan"`: Validates against Vulkan SPIR-V rules (`-V`). Requires explicit layout locations (`layout(location = 0)`), descriptor sets, and push constants.
- **`formatter`** *(string, default: `"clang-format"`)*:
  - `"clang-format"`: Uses external `clang-format` for AST-level indentation and line breaking. If `clang-format` is not found, it seamlessly falls back to `"builtin"`.
  - `"builtin"`: Uses the built-in pure-Rust formatter with zero external dependencies.
- **`glslang_validator_path`** *(string, optional)*: Absolute path to `glslangValidator` executable. Leave empty to automatically search system `PATH` and standard SDK locations.
- **`clang_format_path`** *(string, optional)*: Absolute path to `clang-format` executable. Leave empty to automatically search system `PATH`.

---

### Per-File Inline Directives

You can override settings directly on a per-file basis by placing directives at the top of your shader file:

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

- `// @target: opengl` - Force Desktop OpenGL validation for this file.
- `// @target: vulkan` - Force Vulkan SPIR-V validation for this file.
- `// @formatter: clang-format` - Use clang-format engine for this file.
- `// @formatter: builtin` - Force built-in pure-Rust formatter for this file.

---

## Supported Shader Stages

| Stage | Extensions |
|---|---|
| Vertex / Fragment / Geometry | `.vert`, `.frag`, `.geom` |
| Tessellation Control & Eval | `.tesc`, `.tese` |
| Compute / Mesh / Task | `.comp`, `.mesh`, `.task` |
| Ray Tracing Pipelines | `.rgen`, `.rint`, `.rahit`, `.rchit`, `.rmiss`, `.rcall` |
| Generic GLSL / Headers | `.glsl`, `.glslh` (or any file with `#version \d+`) |

---

## Author & License

- **Author:** Semih Özdemir ([@zyr1on](https://github.com/zyr1on)) - `semihozdmirr@gmail.com`
- **License:** [MIT License](LICENSE)
