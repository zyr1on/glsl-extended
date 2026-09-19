# 🛠️ Building GLSL Extended from Source

If you want to build and hack on GLSL Extended locally, you can easily compile all components from source.

## Prerequisites

- [Rust & Cargo](https://rustup.rs/) (latest stable via `rustup`)
- [Node.js](https://nodejs.org/) (v18+ with npm, for the VS Code extension)
- WebAssembly Target for Zed:
  ```bash
  rustup target add wasm32-wasip2
  ```

---

## 1. Build the Language Server (`glsl_validator`)

To compile the standalone Rust LSP engine:

```bash
cargo build --release --manifest-path glsl_validator/Cargo.toml
```

The compiled binary will be located at:
- **Windows:** `target/release/glsl_validator.exe`
- **Linux / macOS:** `target/release/glsl_validator`

---

## 2. Build the Zed Extension (`.wasm`)

To compile the WebAssembly extension for Zed Editor:

```bash
cargo build --release --target wasm32-wasip2 --manifest-path editors/zed/Cargo.toml
```

The compiled `.wasm` binary will be at:
`target/wasm32-wasip2/release/zed_glsl_extended.wasm`

To package it for Zed:
```bash
mkdir -p dist/package
cp target/wasm32-wasip2/release/zed_glsl_extended.wasm dist/package/extension.wasm
cp editors/zed/extension.toml dist/package/
cp -r editors/zed/languages dist/package/
cd dist/package && zip -r ../../zed-glsl-general-release.zip .
```

---

## 3. Build the VS Code Extension (`.vsix`)

To compile and package the VS Code extension:

```bash
cd editors/vscode
npm install
npm run compile
npx @vscode/vsce package --no-dependencies
```

This produces `vscode-glsl-extended-<version>.vsix` (under 10 KB, zero bundled binaries).

---

## 4. Running Tests

To run all unit and integration tests across the workspace:

```bash
cargo test --workspace
```
