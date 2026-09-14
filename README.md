# GLSL Extended for Zed

Comprehensive GLSL and shader development extension for the Zed Editor, featuring full support for Desktop OpenGL (from `#version 330 core` through `#version 460 core`) and Vulkan (SPIR-V) validation, AST-based formatting, smart vector swizzling, recursive `#include` autocompletion, signature help with docs.gl, and Tree-sitter syntax highlighting.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Zed Extension API](https://img.shields.io/badge/Zed%20Extension%20API-v0.7.0-blue)](https://crates.io/crates/zed_extension_api)
[![Release](https://img.shields.io/github/v/release/zyr1on/zed-glsl-extended?color=green)](https://github.com/zyr1on/zed-glsl-extended/releases)

---

https://github.com/user-attachments/assets/51fef2aa-2fce-43d7-9918-d6d61b49b02c

---

## Features

- **Dual Language Server Architecture:**
  - **LSP 1 (`glsl_analyzer`):** Fast autocompletion, inline hover documentation, and goto-definition.
  - **LSP 2 (`glsl_validator`):** Compiler diagnostics, hybrid formatting, color preview, and signature help.

- **Signature Help & docs.gl Documentation (`signatureHelp`, `hover`):**
  - Parameter hints with active parameter highlighting while typing function arguments.
  - Built-in OpenGL 4.6 documentation (`texture`, `normalize`, `mix`, `clamp`, `dot`, `cross`, etc.) with docs.gl summaries.
  - Resolves user-defined functions across `#include` files with live editor cache and filesystem mtime caching.

- **Recursive `#include` Autocompletion:**
  - Autocomplete functions declared across directly and transitively included files (`common.glsl`, etc.).
  - Zero autocompletion interruptions inside comments (`//`, `/* */`) or string literals.

- **Dual Target API Validation (OpenGL & Vulkan):**
  - **OpenGL (Default):** Validates pure Desktop OpenGL without mandatory SPIR-V layout restrictions across all versions (from `#version 330 core` through `#version 460 core`).
  - **Vulkan (SPIR-V):** Strictly enforces SPIR-V layout locations (`layout(location = 0)`), descriptor sets, and push constants.
  - **Dynamic Diagnostics Source:** Matches actual shader version in error messages (e.g. `glslangValidator (OpenGL 330 core)`).
  - **Zero-Restart Switching:** Toggle dynamically via `settings.json` or per-file `// @target: vulkan` directives.

- **Smart Vector Swizzling & Generic Snippets:**
  - Dimension-aware swizzle completions on vectors (`.xyzw`, `.rgba`, `.stpq`) and `.length()`.
  - Generic boilerplate snippets (`ubo`, `ssbo`, `vert`, `frag`, `comp`, `geom`, `struct`, `func`, `main`).

- **Hybrid Code Formatting (`textDocument/formatting`):**
  - **Clang-Format Engine:** Uses system `clang-format` for AST-level formatting.
  - **Pure-Rust Fallback:** Activates automatically if `clang-format` is not installed.

- **Interactive Color Swatches & Picker:**
  - Inline color previews for `vec3(...)` and `vec4(...)` color literals with visual color picker support.

---

## Requirements

The extension coordinates external tools to provide the full IDE experience:

1. **`glslangValidator` (Required for Compiler Diagnostics):**
   - **Windows:** `winget install KhronosGroup.VulkanSDK` (or via [LunarG Vulkan SDK](https://vulkan.lunarg.com/sdk/home))
   - **Linux:** `sudo apt install glslang-tools` (Ubuntu/Debian) or `sudo pacman -S glslang` (Arch)
   - **macOS:** `brew install glslang`

2. **`glsl_analyzer` (Autocompletion, Hover, Navigation):**
   - Automatically downloaded by Zed on first launch if not found in `PATH`.

3. **`clang-format` (Optional, for AST Formatting):**
   - **Windows:** `winget install LLVM.LLVM`
   - **Linux:** `sudo apt install clang-format` (Ubuntu/Debian) or `sudo pacman -S clang` (Arch)
   - **macOS:** `brew install clang-format`
   *(If not installed, formatting seamlessly falls back to the built-in pure-Rust engine).*

---

## Configuration (`settings.json`)

Open your Zed settings (`Ctrl+,` or `Cmd+,`):

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

### Per-File Inline Directives

Override settings directly in the first few lines of any shader file:

```glsl
// @target: vulkan
// @formatter: clang-format
#version 460 core
```

- `// @target: opengl` - Force Desktop OpenGL validation.
- `// @target: vulkan` - Force Vulkan SPIR-V validation.
- `// @formatter: clang-format` - Use clang-format engine.
- `// @formatter: builtin` - Force built-in pure-Rust formatter.

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
