# GLSL Extended for Zed

Comprehensive GLSL and shader development extension for the Zed Editor, featuring full support for Desktop OpenGL (from `#version 330 core` through `#version 460 core`) and Vulkan (SPIR-V) validation, AST-based formatting, smart vector swizzling, recursive `#include` autocompletion, signature help with docs.gl, and Tree-sitter syntax highlighting.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Zed Extension API](https://img.shields.io/badge/Zed%20Extension%20API-v0.7.0-blue)](https://crates.io/crates/zed_extension_api)
[![Release](https://img.shields.io/github/v/release/zyr1on/zed-glsl-extended?color=green)](https://github.com/zyr1on/zed-glsl-extended/releases)

---

https://github.com/user-attachments/assets/51fef2aa-2fce-43d7-9918-d6d61b49b02c

---

> [!TIP]
> ### Automatic Installation (Zero Setup)
> **Everything is automatic!** GLSL Extended automatically resolves, downloads, and configures the required binaries from official GitHub releases when you open a shader:
> - **`glsl_validator`**: Automatically downloaded from [zed-glsl-extended/releases](https://github.com/zyr1on/zed-glsl-extended/releases).
> - **`glslang` / `glslangValidator`**: Automatically downloaded from official [KhronosGroup/glslang/releases](https://github.com/KhronosGroup/glslang/releases) if not already installed in your system `PATH` or Vulkan SDK.
> - **`glsl_analyzer`**: (Optional external Zig LSP) Automatically downloaded from [nolanderc/glsl_analyzer/releases](https://github.com/nolanderc/glsl_analyzer/releases) if chosen in settings.

> [!NOTE]
> ### Manual Installation (Optional / Offline Environments)
> If you prefer using your own system compiler, or work in an offline / air-gapped environment, GLSL Extended will seamlessly prioritize your local binaries from `PATH`:
> 
> - **Windows:**
>   - **winget / Vulkan SDK:** `winget install KhronosGroup.VulkanSDK` (or via [LunarG Vulkan SDK](https://vulkan.lunarg.com/sdk/home))
>   - **MSYS2 (UCRT64 / MINGW64):** `pacman -S mingw-w64-ucrt-x86_64-glslang`
>   - **Pre-built Binaries:** Download zip from [KhronosGroup/glslang/releases](https://github.com/KhronosGroup/glslang/releases) and add `bin/` to `PATH`.
> 
> - **Linux:**
>   - **Ubuntu / Debian:** `sudo apt install glslang-tools`
>   - **Arch Linux:** `sudo pacman -S glslang`
>   - **Fedora:** `sudo dnf install glslang`
>   - **Pre-built Tarball:** Download from [KhronosGroup/glslang/releases](https://github.com/KhronosGroup/glslang/releases).
> 
> - **macOS:**
>   - **Homebrew:** `brew install glslang`
>   - **Pre-built Universal Binary:** Download from [KhronosGroup/glslang/releases](https://github.com/KhronosGroup/glslang/releases).

---

## Features

- **Unified Language Server:** `glsl_validator` provides compile diagnostics, variable/function autocompletion with snippets, signature help, hover docs, formatting, and goto-definition.
- **Smart Variable & Function Autocompletion:** Autocompletes user variables (`out vec3 FragPos`, `in`, `uniform`, `struct`, `const`, `#define`) and functions with automatic `()` parentheses placement.
- **Goto Definition (`F12`):** Jump directly to definitions of functions, structs, and variables across current and `#include` files.
- **Signature Help & docs.gl Hover:** Real-time parameter hints and built-in OpenGL 4.6 documentation summaries with parameter specs.
- **Recursive `#include` Navigation:** Suggests functions, variables, and structs across all included files; automatically suppressed in comments and strings.
- **Dual Target API Validation:** Validates Desktop OpenGL (`#version 330 core` to `460 core`) and Vulkan SPIR-V with dynamic version labels.
- **Smart Vector Swizzling:** Context-aware `.xyzw`, `.rgba`, and `.stpq` swizzle completions on vectors.
- **GLSL Boilerplate Snippets:** Generic templates for `ubo`, `ssbo`, shaders, structs, and functions.
- **Hybrid Code Formatting:** AST-level `clang-format` engine with automatic built-in pure-Rust fallback.
- **Interactive Color Swatches & Picker:** Inline visual color previews and interactive picker for `vec3` and `vec4`.
- **Tree-sitter Syntax Highlighting:** Highlighting for 400+ types, built-in variables, and proper brace indentation.

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
      "binary": {
        "path": ""
      },
      "initialization_options": {
        "target_api": "opengl",
        "formatter": "clang-format",
        "glslang_validator_path": "",
        "glsl_validator_path": "",
        "clang_format_path": ""
      }
    },
    "glsl_analyzer": {
      "binary": {
        "path": ""
      },
      "initialization_options": {
        "glsl_analyzer_path": ""
      }
    }
  }
}
```

### Configuration Options

| Option | Values / Default | Description |
|---|---|---|
| `languages.GLSL.tab_size` | `4` *(or `2`, `8`)* | Indentation space width. |
| `languages.GLSL.format_on_save` | `"off"` *(default)* / `"on"` | Format document automatically on save. |
| `languages.GLSL.formatter` | `glsl_validator` | Routes formatting to `glsl_validator` language server. |
| `target_api` | `"opengl"` *(default)* / `"vulkan"` | `"opengl"` (Desktop OpenGL, respects `#version` 330–460) or `"vulkan"` (strict SPIR-V). |
| `formatter` | `"clang-format"` *(default)* / `"builtin"` | `"clang-format"` (AST-level, auto-fallback) or `"builtin"` (pure-Rust). |
| `glslang_validator_path` | `""` *(optional)* | Custom executable path to `glslang` or `glslangValidator`. Leave empty/null for auto-discovery and automatic Khronos download. |
| `glsl_validator_path` | `""` *(optional)* | Custom executable path to `glsl_validator` (or configure via `lsp.glsl_validator.binary.path`). Leave empty/null for auto-discovery. |
| `glsl_analyzer_path` | `""` *(optional)* | Custom executable path to `glsl_analyzer` (or configure via `lsp.glsl_analyzer.binary.path`). Leave empty/null for auto-discovery. |
| `clang_format_path` | `""` *(optional)* | Custom executable path to `clang-format`. Leave empty/null for auto-discovery. |


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
