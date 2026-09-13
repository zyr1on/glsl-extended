# GLSL Extended — Zed Editor Extension

Comprehensive, high-performance GLSL and shader development extension for the **Zed Editor**, specifically tailored for **OpenGL 4.6 (Core Profile)**.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Zed Extension API](https://img.shields.io/badge/Zed%20Extension%20API-v0.7.0-blue)](https://crates.io/crates/zed_extension_api)

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

- **Pure Desktop OpenGL 4.6 Semantics:**
  - No mandatory SPIR-V restrictions: declarations like `out vec3 Normal;` compile without false `location` errors.
  - Precise line-and-column diagnostic squiggly underlines on syntax or type errors.

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

## Requirements

The extension integrates with two core command-line tools:

1. **`glsl_analyzer`**: Language server for autocomplete and hover.
   - Automatically downloaded by Zed on first launch if not already found in your `PATH`.
   - Pre-built releases: [nolanderc/glsl_analyzer](https://github.com/nolanderc/glsl_analyzer/releases).

2. **`glslangValidator`**: The Khronos reference GLSL compiler used for diagnostics.
   - **Windows:** Included in the [Vulkan SDK](https://vulkan.lunarg.com/sdk/home) or MSYS2 (`pacman -S mingw-w64-ucrt-x86_64-glslang`).
   - **Linux:** `sudo apt install glslang-tools` (Ubuntu/Debian) or `sudo pacman -S glslang` (Arch).
   - **macOS:** `brew install glslang`.

3. **`glsl_validator`**: Automatically downloaded from GitHub Releases by Zed on first launch. Can also be built from source using `cargo install --path glsl_validator`.

---

## Installation

### Installing as a Dev Extension in Zed

1. Clone or download this repository:
   ```bash
   git clone https://github.com/zyr1on/zed-glsl-extended.git
   ```
2. Open **Zed**.
3. Open the Command Palette (`Ctrl+Shift+P` on Windows/Linux, `Cmd+Shift+P` on macOS).
4. Type and select: **`zed: install dev extension`**.
5. Choose the cloned `zed-glsl-extended` directory.
6. Zed will compile the WebAssembly component and activate the extension immediately.

### Configuration (`settings.json`)

Add or verify the following configuration in your Zed settings (`Ctrl+,` or `%APPDATA%\Zed\settings.json`):

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
├── .gitignore                   # Git ignore patterns
└── README.md                    # Documentation
```

---

## Troubleshooting

- **Zed logs:** Open Command Palette → `zed: open log`.
- **Diagnostics bridge logs:** `%LOCALAPPDATA%\Temp\glsl_validator.log` (Windows) or `/tmp/glsl_validator.log` (Linux/macOS).

---

## Author

- **Semih Özdemir** ([@zyr1on](https://github.com/zyr1on)) — `semihozdmirr@gmail.com`

---

## License

This project is licensed under the [MIT License](LICENSE).