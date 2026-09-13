// src/lib.rs
//
// GLSL Extended for Zed Editor
// =====================================================================
// Target: OpenGL 4.6 (Core Profile)
// LSP 1 : glsl_analyzer  (Autocomplete / Hover / Goto-Definition)
// LSP 2 : glsl_validator (glslangValidator compile diagnostics / linting)
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
        let asset_name = format!(
            "{arch}-{os}.zip",
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X86    => "x86",
                zed::Architecture::X8664  => "x86_64",
            },
            os = match platform {
                zed::Os::Mac     => "macos",
                zed::Os::Linux   => "linux",
                zed::Os::Windows => "windows",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("Asset '{asset_name}' not found in glsl_analyzer release"))?;

        let version_dir = format!("glsl_analyzer-{}", release.version);
        let exe = if matches!(platform, zed::Os::Windows) { ".exe" } else { "" };
        let binary_path = format!("{version_dir}/glsl_analyzer{exe}");

        if !fs::metadata(&binary_path).is_ok_and(|s| s.is_file()) {
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

            zed::make_file_executable(&binary_path)?;
        }

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

        // 3) Common Windows MSYS2 / UCRT64 path
        let msys = "C:\\msys64\\ucrt64\\bin\\glsl_validator.exe";
        if fs::metadata(msys).is_ok_and(|s| s.is_file()) {
            return Ok(msys.to_string());
        }

        // 4) Try downloading pre-built binary from repository GitHub Releases
        let (platform, arch) = zed::current_platform();
        let ext = if matches!(platform, zed::Os::Windows) { "zip" } else { "tar.gz" };
        let file_type = if matches!(platform, zed::Os::Windows) {
            zed::DownloadedFileType::Zip
        } else {
            zed::DownloadedFileType::GzipTar
        };

        let asset_name = format!(
            "glsl_validator-{arch}-{os}.{ext}",
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X86    => "x86",
                zed::Architecture::X8664  => "x86_64",
            },
            os = match platform {
                zed::Os::Mac     => "macos",
                zed::Os::Linux   => "linux",
                zed::Os::Windows => "windows",
            }
        );

        if let Ok(release) = zed::latest_github_release(
            "zyr1on/zed-glsl-extended",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) {
            if let Some(asset) = release.assets.iter().find(|a| a.name == asset_name) {
                let version_dir = format!("glsl_validator-{}", release.version);
                let exe = if matches!(platform, zed::Os::Windows) { ".exe" } else { "" };
                let binary_path = format!("{version_dir}/glsl_validator{exe}");

                if !fs::metadata(&binary_path).is_ok_and(|s| s.is_file()) {
                    let _ = zed::download_file(&asset.download_url, &version_dir, file_type);
                    let _ = zed::make_file_executable(&binary_path);
                }

                if fs::metadata(&binary_path).is_ok_and(|s| s.is_file()) {
                    self.cached_glsl_validator = Some(binary_path.clone());
                    return Ok(binary_path);
                }
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