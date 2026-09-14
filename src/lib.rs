// src/lib.rs
//
// GLSL Extended for Zed Editor
// =====================================================================
// Target: OpenGL 4.6 (Core Profile)
// LSP 1 : glsl_validator (Default: Diagnostics, variables, functions with (),
//                         snippets, signature help, hover docs, formatting, goto-def)
// LSP 2 : glsl_analyzer  (Optional: External Zig LSP)
// =====================================================================

use std::fs;
use zed::settings::LspSettings;
use zed_extension_api::{self as zed, LanguageServerId, Result, serde_json};

struct GlslExtendedExtension {
    cached_glsl_analyzer: Option<String>,
    cached_glsl_validator: Option<String>,
}

impl GlslExtendedExtension {
    /// Locates or downloads the glsl_analyzer language server binary.
    fn find_glsl_analyzer(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // 1) Check PATH
        if let Some(path) = worktree.which("glsl_analyzer") {
            return Ok(path);
        }

        // 2) Check cached binary
        if let Some(path) = &self.cached_glsl_analyzer
            && fs::metadata(path).is_ok_and(|s| s.is_file())
        {
            return Ok(path.clone());
        }

        // 3) Download automatically from GitHub Releases
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "nolanderc/glsl_analyzer",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = match (platform, arch) {
            (zed::Os::Windows, zed::Architecture::X8664) => "x86_64-windows.zip",
            (zed::Os::Windows, zed::Architecture::Aarch64) => "aarch64-windows.zip",
            (zed::Os::Linux, zed::Architecture::X8664) => "x86_64-linux-musl.zip",
            (zed::Os::Linux, zed::Architecture::Aarch64) => "aarch64-linux-musl.zip",
            (zed::Os::Mac, zed::Architecture::Aarch64) => "aarch64-macos.zip",
            (zed::Os::Mac, zed::Architecture::X8664) => "x86_64-macos.zip",
            _ => return Err("Unsupported platform or architecture for glsl_analyzer".to_string()),
        };

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("Asset '{asset_name}' not found in glsl_analyzer release"))?;

        let version_dir = format!("glsl_analyzer-{}", release.version);
        let exe = if matches!(platform, zed::Os::Windows) {
            ".exe"
        } else {
            ""
        };
        let candidate_bin = format!("{version_dir}/bin/glsl_analyzer{exe}");
        let candidate_root = format!("{version_dir}/glsl_analyzer{exe}");

        if !fs::metadata(&candidate_bin).is_ok_and(|s| s.is_file())
            && !fs::metadata(&candidate_root).is_ok_and(|s| s.is_file())
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|e| format!("Failed to download glsl_analyzer: {e}"))?;
        }

        let binary_path = if fs::metadata(&candidate_bin).is_ok_and(|s| s.is_file()) {
            candidate_bin
        } else if fs::metadata(&candidate_root).is_ok_and(|s| s.is_file()) {
            candidate_root
        } else {
            return Err(format!(
                "glsl_analyzer binary not found in '{candidate_bin}' or '{candidate_root}'"
            ));
        };

        let _ = zed::make_file_executable(&binary_path);

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        self.cached_glsl_analyzer = Some(binary_path.clone());
        Ok(binary_path)
    }

    /// Locates or downloads the glsl_validator language server binary.
    fn find_glsl_validator(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // 1) Check PATH (Windows, Linux, macOS)
        if let Some(path) = worktree.which("glsl_validator") {
            return Ok(path);
        }

        // 2) Check cached binary
        if let Some(path) = &self.cached_glsl_validator
            && fs::metadata(path).is_ok_and(|s| s.is_file())
        {
            return Ok(path.clone());
        }

        let (platform, arch) = zed::current_platform();

        // 3) Check user cargo bin directory (~/.cargo/bin/glsl_validator)
        let exe = if matches!(platform, zed::Os::Windows) {
            ".exe"
        } else {
            ""
        };
        if let Ok(home) = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")) {
            let cargo_bin = format!("{home}/.cargo/bin/glsl_validator{exe}");
            if fs::metadata(&cargo_bin).is_ok_and(|s| s.is_file()) {
                return Ok(cargo_bin);
            }
        }

        // 4) Common fallback locations per platform
        let fallbacks: &[&str] = match platform {
            zed::Os::Windows => &[
                "C:\\msys64\\ucrt64\\bin\\glsl_validator.exe",
                "C:\\msys64\\mingw64\\bin\\glsl_validator.exe",
                "C:\\msys64\\clang64\\bin\\glsl_validator.exe",
            ],
            zed::Os::Linux | zed::Os::Mac => {
                &["/usr/local/bin/glsl_validator", "/usr/bin/glsl_validator"]
            }
        };
        for fallback in fallbacks {
            if fs::metadata(fallback).is_ok_and(|s| s.is_file()) {
                return Ok(fallback.to_string());
            }
        }

        // 5) Try downloading pre-built binary from repository GitHub Releases
        let ext = if matches!(platform, zed::Os::Windows) {
            "zip"
        } else {
            "tar.gz"
        };
        let file_type = if matches!(platform, zed::Os::Windows) {
            zed::DownloadedFileType::Zip
        } else {
            zed::DownloadedFileType::GzipTar
        };

        let asset_name = format!(
            "glsl_validator-{arch}-{os}.{ext}",
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X86 => "x86",
                zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "macos",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "windows",
            }
        );

        if let Ok(release) = zed::latest_github_release(
            "zyr1on/zed-glsl-extended",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) && let Some(asset) = release.assets.iter().find(|a| a.name == asset_name)
        {
            let version_dir = format!("glsl_validator-{}", release.version);
            let candidate_root = format!("{version_dir}/glsl_validator{exe}");
            let candidate_bin = format!("{version_dir}/bin/glsl_validator{exe}");

            if !fs::metadata(&candidate_root).is_ok_and(|s| s.is_file())
                && !fs::metadata(&candidate_bin).is_ok_and(|s| s.is_file())
            {
                let _ = zed::download_file(&asset.download_url, &version_dir, file_type);
            }

            let binary_path = if fs::metadata(&candidate_root).is_ok_and(|s| s.is_file()) {
                Some(candidate_root)
            } else if fs::metadata(&candidate_bin).is_ok_and(|s| s.is_file()) {
                Some(candidate_bin)
            } else {
                None
            };

            if let Some(path) = binary_path {
                let _ = zed::make_file_executable(&path);
                self.cached_glsl_validator = Some(path.clone());
                return Ok(path);
            }
        }

        Err("glsl_validator binary not found. Please add 'glsl_validator' to your PATH or install it via 'cargo install --path glsl_validator'.".to_string())
    }
}

impl zed::Extension for GlslExtendedExtension {
    fn new() -> Self {
        Self {
            cached_glsl_analyzer: None,
            cached_glsl_validator: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            "glsl_analyzer" => Ok(zed::Command {
                command: self.find_glsl_analyzer(language_server_id, worktree)?,
                args: vec![],
                env: Default::default(),
            }),
            "glsl_validator" => Ok(zed::Command {
                command: self.find_glsl_validator(language_server_id, worktree)?,
                args: vec![],
                env: Default::default(),
            }),
            unknown => Err(format!("Unknown language server: {unknown}")),
        }
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let server_name = language_server_id.as_ref();
        let settings = LspSettings::for_worktree(server_name, worktree)?;
        if let Some(opts) = settings.initialization_options {
            return Ok(Some(opts));
        }

        if server_name == "glsl_analyzer" {
            return Ok(Some(serde_json::json!({
                "validateOnType": true,
                "maxNumberOfProblems": 200,
                "targetClientVersion": "opengl460"
            })));
        }

        Ok(None)
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        Ok(settings.settings)
    }
}

zed::register_extension!(GlslExtendedExtension);
