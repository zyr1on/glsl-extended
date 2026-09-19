# GLSL Extended (for Zed & VS Code)

Comprehensive GLSL and shader development extension for **Zed Editor** and **Visual Studio Code**, featuring full support for Desktop OpenGL (from `#version 330 core` through `#version 460 core`) and Vulkan (SPIR-V) validation, AST-based formatting, smart vector swizzling, recursive `#include` autocompletion, signature help with docs.gl, and syntax highlighting.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Zed Extension API](https://img.shields.io/badge/Zed%20Extension%20API-v0.7.0-blue)](https://crates.io/crates/zed_extension_api)
[![Release](https://img.shields.io/github/v/release/zyr1on/zed-glsl-extended?color=green)](https://github.com/zyr1on/zed-glsl-extended/releases)

---

https://github.com/user-attachments/assets/51fef2aa-2fce-43d7-9918-d6d61b49b02c

---

> [!TIP]
> ### 📦 Quick Install: Zed Editor
> 1. Download **`zed-glsl-general-release.zip`** from [Latest Releases](https://github.com/zyr1on/zed-glsl-extended/releases) and extract it anywhere on your computer.
> 2. Open Zed and open the Extensions panel (`Ctrl+Shift+X` on Windows/Linux, `Cmd+Shift+X` on macOS).
> 3. Click **"Install Dev Extension"** at the top right and select the extracted folder.
> 
> *Done! The extension will load immediately without requiring Rust or Cargo.*

> [!TIP]
> ### 📦 Quick Install: Visual Studio Code
> 1. Download **`vscode-glsl-general-release.vsix`** from [Latest Releases](https://github.com/zyr1on/zed-glsl-extended/releases).
> 2. In VS Code, open Extensions (`Ctrl+Shift+X` / `Cmd+Shift+X`), click the **`...`** (Views and More Actions) menu at the top of the Extensions panel, and select **"Install from VSIX..."**.
> 3. Select the downloaded `vscode-glsl-general-release.vsix` file.
> 
> *Done! VS Code will automatically download the language server binary in the background upon opening a shader.*

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

## Extra Configuration (`settings.json`)

### Zed Settings (`settings.json`)

To configure GLSL settings in Zed, open your settings (`Ctrl+,` on Windows/Linux, `Cmd+,` on macOS):

```json
{
  "languages": {
    "GLSL": {
      "tab_size": 4,                        // Indentation space width (e.g., 2, 4, 8)
      "format_on_save": "off",              // Auto-format shader on save ("off" | "on")
      "formatter": {
        "language_server": {
          "name": "glsl_validator"          // Route formatting to glsl_validator LSP
        }
      }
    }
  },
  "lsp": {
    "glsl_validator": {
      "initialization_options": {
        "target_api": "opengl",             // Target validation: "opengl" (Desktop 330-460) or "vulkan" (SPIR-V)
        "formatter": "clang-format",        // Formatter engine: "clang-format" (AST-level) or "builtin" (pure-Rust)
        "glslang_validator_path": "",       // Custom path to glslangValidator (empty for auto-download / PATH)
        "glsl_validator_path": "",          // Custom path to glsl_validator (empty for auto-download / PATH)
        "clang_format_path": ""             // Custom path to clang-format (empty for auto-discovery)
      }
    },
    "glsl_analyzer": {
      "initialization_options": {
        "glsl_analyzer_path": ""            // Optional custom path to external glsl_analyzer LSP
      }
    }
  }
}
```

### Visual Studio Code Settings (`settings.json`)

To configure GLSL settings in VS Code, open your User or Workspace `settings.json` (`Ctrl+Shift+P` -> `Preferences: Open User Settings (JSON)`):

```json
{
  "[glsl]": {
    "editor.tabSize": 4,                    // Indentation space width (e.g., 2, 4, 8)
    "editor.formatOnSave": false            // Auto-format shader document on save (true | false)
  },
  "glslExtended.validatorPath": "",          // Custom executable path to glsl_validator (empty for auto-download / PATH)
  "glslExtended.glslangValidatorPath": "",   // Custom executable path to Khronos glslangValidator (empty for auto-download / PATH)
  "glslExtended.analyzerPath": "",           // Optional custom executable path to glsl_analyzer
  "glslExtended.trace.server": "off"         // Trace LSP communication in Output panel ("off" | "messages" | "verbose")
}
```


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

## 💻 Using `glsl_validator` with Other Editors (Neovim, Helix, Sublime Text)

The core language server (`glsl_validator`) is distributed as a standalone native binary and can be used with any editor that supports the Language Server Protocol (LSP).

### Prerequisites
1. **Download `glsl_validator`:** Download the pre-built native binary for your OS (Windows, Linux, macOS) from [Latest Releases](https://github.com/zyr1on/glsl-extended/releases) and place it in your system `PATH`.
2. **Install `glslang`:** Ensure `glslangValidator` (or `glslang`) is available in your `PATH` (e.g., via Vulkan SDK, `apt install glslang-tools`, `pacman -S glslang`, or `brew install glslang`).
3. *(Optional)* **`glsl_analyzer`**: If you wish to route semantic analysis to `glsl_analyzer`, ensure it is installed in your `PATH`.

### Neovim (`nvim-lspconfig`)

Add `glsl_validator` to your Neovim configuration:

```lua
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

if not configs.glsl_validator then
  configs.glsl_validator = {
    default_config = {
      cmd = { 'glsl_validator' },
      filetypes = { 'glsl', 'vert', 'frag', 'geom', 'comp', 'tesc', 'tese', 'rgen', 'rmiss', 'rchit' },
      root_dir = lspconfig.util.root_pattern('.git', 'compile_flags.txt'),
      init_options = {
        target_api = 'opengl',          -- 'opengl' or 'vulkan'
        formatter = 'clang-format',     -- 'clang-format' or 'builtin'
        -- glslang_validator_path = '/custom/path/to/glslangValidator',
      },
    },
  }
end

lspconfig.glsl_validator.setup({})
```

### Helix (`languages.toml`)

In `~/.config/helix/languages.toml`:

```toml
[language-server.glsl_validator]
command = "glsl_validator"
config = { target_api = "opengl", formatter = "clang-format" }

[[language]]
name = "glsl"
scope = "source.glsl"
file-types = ["glsl", "vert", "frag", "geom", "comp", "tesc", "tese", "rgen", "rmiss", "rchit"]
language-servers = [ "glsl_validator" ]
```

### Sublime Text (LSP Package)

In `Preferences -> Package Settings -> LSP -> Settings`:

```json
{
  "clients": {
    "glsl_validator": {
      "enabled": true,
      "command": ["glsl_validator"],
      "selector": "source.glsl",
      "initializationOptions": {
        "target_api": "opengl",
        "formatter": "clang-format"
      }
    }
  }
}
```

---

## 🛠️ Building from Source

If you want to build and hack on GLSL Extended locally, you can easily compile all components from source:

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) (latest stable via `rustup`)
- [Node.js](https://nodejs.org/) (v18+ with npm, for the VS Code extension)
- WebAssembly Target for Zed:
  ```bash
  rustup target add wasm32-wasip2
  ```

### 1. Build the Language Server (`glsl_validator`)
To compile the standalone Rust LSP engine:
```bash
cargo build --release --manifest-path glsl_validator/Cargo.toml
```
The compiled binary will be located at:
- **Windows:** `target/release/glsl_validator.exe`
- **Linux / macOS:** `target/release/glsl_validator`

### 2. Build the Zed Extension (`.wasm`)
To compile the WebAssembly extension for Zed Editor:
```bash
cargo build --release --target wasm32-wasip2 --manifest-path editors/zed/Cargo.toml
```
The compiled `.wasm` binary will be at:
`target/wasm32-wasip2/release/zed_glsl_extended.wasm`

### 3. Build the VS Code Extension (`.vsix`)
To compile and package the VS Code extension:
```bash
cd editors/vscode
npm install
npm run compile
npx @vscode/vsce package --no-dependencies
```
This produces `vscode-glsl-extended-<version>.vsix` (under 10 KB, zero bundled binaries).

### 4. Running Tests
To run all unit and integration tests across the workspace:
```bash
cargo test --workspace
```

---

## Author & License

- **Author:** Semih Özdemir ([@zyr1on](https://github.com/zyr1on)) - `semihozdmirr@gmail.com`
- **License:** [MIT License](LICENSE)
