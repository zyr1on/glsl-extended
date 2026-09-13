# GLSL Extended for Zed

High-performance GLSL and shader language support for the Zed Editor, featuring OpenGL 4.6 (Core Profile) and Vulkan (SPIR-V) validation, AST-based formatting, smart vector swizzling, snippets, and tree-sitter syntax highlighting.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Zed Extension API](https://img.shields.io/badge/Zed%20Extension%20API-v0.7.0-blue)](https://crates.io/crates/zed_extension_api)
[![Release](https://img.shields.io/github/v/release/zyr1on/zed-glsl-extended?color=green)](https://github.com/zyr1on/zed-glsl-extended/releases)

---

## Configuration (`settings.json`)

Add the following to your Zed settings (`Ctrl+,` or Command Palette -> `zed: open settings`):

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

| Setting | Field in `settings.json` | Default | Alternatives | Description |
|---|---|---|---|---|
| **Target API** | `lsp.glsl_validator.initialization_options.target_api` | `"opengl"` | `"vulkan"` | `"opengl"` validates Desktop OpenGL 4.6 (`out vec3 Normal;` compiles without SPIR-V restrictions). `"vulkan"` strictly validates SPIR-V rules (mandatory `layout(location = 0)`). |
| **Formatter Engine** | `lsp.glsl_validator.initialization_options.formatter` | `"clang-format"` | `"builtin"` | `"clang-format"` uses system `clang-format` with AST parsing and auto-fallback to built-in. `"builtin"` forces the internal zero-dependency Rust formatter. |
| **Format On Save** | `languages.GLSL.format_on_save` | `"off"` | `"on"` | When `"off"`, formatting only runs on demand (`Ctrl+K, Ctrl+F`). When `"on"`, files format automatically on `Ctrl+S`. |
| **Tab Size** | `languages.GLSL.tab_size` | `4` | `2`, `4`, `8` | Indentation width in spaces. |

### Per-File Inline Directives

You can override settings per shader file without changing your global settings. Place directives in the first few lines of your shader:

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

Available inline directives:
- `// @target: opengl` - Force Desktop OpenGL 4.6 validation.
- `// @target: vulkan` - Force Vulkan SPIR-V validation.
- `// @formatter: clang-format` - Use clang-format engine.
- `// @formatter: builtin` - Use built-in pure-Rust engine.

---

## Features

- **Tree-sitter Syntax Highlighting:** 400+ GLSL types (`vec2`-`dmat4`, samplers, images, atomic counters), built-in variables (`gl_Position`, `gl_FragCoord`, etc.), and functions.
- **Dual Language Servers:**
  - `glsl_analyzer`: Autocompletion, hover documentation, and goto-definition.
  - `glsl_validator`: Asynchronous compiler diagnostics via `glslangValidator`, vector swizzling, snippets, and formatting.
- **Smart Vector Swizzling:** Context-aware completions for `.xyzw`, `.rgba`, `.stpq`, and `.length()` on variables, struct members (`t.a.`), and chained swizzles (`t.a.xyz.`).
- **Generic GLSL Snippets:** Skeletons for `ubo`, `ssbo`, `vert`, `frag`, `comp`, `geom`, `struct`, `func`, and `main`.
- **Hybrid Code Formatter:** Primary `clang-format` integration for AST-level indentation, multi-line statement merging, and spacing, with automatic fallback to pure-Rust formatting.
- **Multi-File `#include` Support:** Two-stage preprocessor resolves `#include` directives across shader folders, `include/` directories, and parent directories.
- **Document Color Swatches:** Inline color previews and color picker support for `vec3(...)` and `vec4(...)` color literals.

---

## Supported File Extensions

| Stage | Extensions |
|---|---|
| Vertex Shader | `.vert` |
| Fragment Shader | `.frag` |
| Geometry Shader | `.geom` |
| Tessellation Control & Eval | `.tesc`, `.tese` |
| Compute Shader | `.comp` |
| Mesh & Task | `.mesh`, `.task` |
| Ray Tracing | `.rgen`, `.rint`, `.rahit`, `.rchit`, `.rmiss`, `.rcall` |
| Headers & Generic GLSL | `.glsl`, `.glslh` |

Any file starting with `#version \d+` is also automatically recognized as GLSL.

---

## Requirements

The extension coordinates two core external tools:

1. **`glsl_analyzer`** (Autocomplete / Navigation):
   - Automatically downloaded by Zed on first run, or resolved from system `PATH`.
2. **`glslangValidator`** (Compile Diagnostics):
   - Provided by the [Vulkan SDK](https://vulkan.lunarg.com/sdk/home), MSYS2 (`pacman -S mingw-w64-ucrt-x86_64-glslang`), or system package manager.
3. **`clang-format`** (Optional, for AST Formatting):
   - Provided by LLVM/Clang or MSYS2 (`pacman -S mingw-w64-ucrt-x86_64-clang-tools-extra`). If absent, the built-in pure-Rust formatter activates automatically.

---

## Installation

### Dev Extension (Local)

1. Clone repository:
   ```bash
   git clone https://github.com/zyr1on/zed-glsl-extended.git
   ```
2. In Zed, open Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`).
3. Select **`zed: install dev extension`** and choose the cloned directory.

---

## Keybindings & Commands

- **Format Document:** Command Palette -> `editor: format` (or `Shift+Alt+F`).
- **Format Selection:** Select code -> `editor: format_selections` (or `Ctrl+K, Ctrl+F`).
- **Save without Formatting:** Default behavior when `"format_on_save": "off"`.

---

## Author

- **Semih Özdemir** ([@zyr1on](https://github.com/zyr1on)) - `semihozdmirr@gmail.com`

---

## License

This project is licensed under the [MIT License](LICENSE).
