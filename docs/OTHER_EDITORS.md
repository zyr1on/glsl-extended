# Using `glsl_validator` with Other Editors

The core language server (`glsl_validator`) is distributed as a standalone native binary and can be used with any editor that supports the Language Server Protocol (LSP).

## Prerequisites

1. **Download `glsl_validator`:** Download the pre-built native binary for your OS (Windows, Linux, macOS) from [Latest Releases](https://github.com/zyr1on/glsl-extended/releases) and place it in your system `PATH`.
2. **Install `glslang`:** Ensure `glslangValidator` (or `glslang`) is available in your `PATH` (e.g., via Vulkan SDK, `apt install glslang-tools`, `pacman -S glslang`, or `brew install glslang`).
3. *(Optional)* **`glsl_analyzer`**: If you wish to route semantic analysis to `glsl_analyzer`, ensure it is installed in your `PATH`.

> [!IMPORTANT]
> ### `glslangValidator` is Required for Diagnostics
> Unlike the **Zed** and **VS Code** extensions—which automatically download compiler binaries in the background—standalone editor setups require you to have **`glslangValidator`** (or `glslang`) installed on your system. 
> 
> Without `glslangValidator`, compiler diagnostics cannot execute. You can install it via:
> - **Windows:** `winget install KhronosGroup.VulkanSDK` (or download the [LunarG Vulkan SDK](https://vulkan.lunarg.com/sdk/home))
> - **Linux:** `sudo apt install glslang-tools` / `sudo pacman -S glslang` / `sudo dnf install glslang`
> - **macOS:** `brew install glslang`

> [!NOTE]
> ### `glsl_analyzer` is Optional
> `glsl_validator` contains its own built-in pure-Rust language engine for autocomplete, swizzling, signature help, and document outline:
> - If `glsl_analyzer` is found in your `PATH`, `glsl_validator` will optionally multiplex with it for enhanced semantic analysis.
> - If `glsl_analyzer` is **not** installed, `glsl_validator` automatically uses its built-in engine with zero functionality loss.

---

## Neovim (`nvim-lspconfig`)

Add `glsl_validator` to your Neovim configuration (e.g., in `init.lua` or your LSP configuration file):

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

---

## Helix (`languages.toml`)

In your Helix language configuration (`~/.config/helix/languages.toml` on Linux/macOS or `%APPDATA%\helix\languages.toml` on Windows):

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

---

## Sublime Text (LSP Package)

1. Install the `LSP` package via Package Control.
2. In `Preferences -> Package Settings -> LSP -> Settings`:

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

## Generic LSP Client Configuration

For any other editor (Emacs `lsp-mode` / `eglot`, Kate, Nova, etc.):

- **Executable:** `glsl_validator` (communicates over standard `stdin` / `stdout`).
- **Filetypes:** `.vert`, `.frag`, `.geom`, `.comp`, `.tesc`, `.tese`, `.mesh`, `.task`, `.rgen`, `.rint`, `.rahit`, `.rchit`, `.rmiss`, `.rcall`, `.glsl`, `.glslh`.
- **Initialization Options (JSON):**
  ```json
  {
    "target_api": "opengl",           // "opengl" | "vulkan"
    "formatter": "clang-format",      // "clang-format" | "builtin"
    "glslang_validator_path": "",     // optional custom path
    "glsl_validator_path": "",        // optional custom path
    "clang_format_path": ""           // optional custom path
  }
  ```
