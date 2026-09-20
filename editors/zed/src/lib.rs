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

/// Extracts the first non-empty, trimmed string matching any key in `keys` from a JSON value.
fn extract_path_from_json(val: &serde_json::Value, keys: &[&str]) -> Option<String> {
    for &key in keys {
        if let Some(v) = val.get(key)
            && let Some(s) = v.as_str()
        {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}

/// Resolves a user-configured path if non-empty, verifying file existence or resolving via PATH.
fn resolve_configured_path(configured: Option<String>, worktree: &zed::Worktree) -> Option<String> {
    let path = configured?;
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return None;
    }
    if fs::metadata(trimmed).is_ok_and(|s| s.is_file()) {
        return Some(trimmed.to_string());
    }
    if let Some(resolved) = worktree.which(trimmed) {
        return Some(resolved);
    }
    Some(trimmed.to_string())
}

struct GlslExtendedExtension {
    cached_glsl_analyzer: Option<String>,
    cached_glsl_validator: Option<String>,
    cached_glslang: Option<String>,
}

impl GlslExtendedExtension {
    /// Locates or downloads the glsl_analyzer language server binary.
    fn find_glsl_analyzer(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // 0) User explicit configuration in Zed settings.json
        if let Ok(settings) = LspSettings::for_worktree(language_server_id.as_ref(), worktree) {
            let configured = {
                let keys = ["glsl_analyzer_path", "analyzer_path", "path"];
                settings
                    .initialization_options
                    .as_ref()
                    .and_then(|opts| extract_path_from_json(opts, &keys))
                    .or_else(|| {
                        settings
                            .settings
                            .as_ref()
                            .and_then(|s| extract_path_from_json(s, &keys))
                    })
                    .or_else(|| {
                        settings
                            .binary
                            .and_then(|b| b.path)
                            .filter(|p| !p.trim().is_empty())
                    })
            };

            if let Some(path) = resolve_configured_path(configured, worktree) {
                return Ok(path);
            }
        }

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
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // 0) User explicit configuration in Zed settings.json
        if let Ok(settings) = LspSettings::for_worktree(language_server_id.as_ref(), worktree) {
            let configured = {
                let keys = ["glsl_validator_path", "validator_path", "path"];
                settings
                    .initialization_options
                    .as_ref()
                    .and_then(|opts| extract_path_from_json(opts, &keys))
                    .or_else(|| {
                        settings
                            .settings
                            .as_ref()
                            .and_then(|s| extract_path_from_json(s, &keys))
                    })
                    .or_else(|| {
                        settings
                            .binary
                            .and_then(|b| b.path)
                            .filter(|p| !p.trim().is_empty())
                    })
            };

            if let Some(path) = resolve_configured_path(configured, worktree) {
                return Ok(path);
            }
        }

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
            "zyr1on/glsl-extended",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) && let Some(asset) = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .or_else(|| {
                if matches!((platform, arch), (zed::Os::Windows, zed::Architecture::Aarch64)) {
                    release.assets.iter().find(|a| a.name == "glsl_validator-x86_64-windows.zip")
                } else {
                    None
                }
            })
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

    /// Locates or downloads the glslang / glslangValidator reference compiler from KhronosGroup.
    fn find_glslang(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // 0) User explicit configuration in Zed settings.json
        if let Ok(settings) = LspSettings::for_worktree("glsl_validator", worktree) {
            let keys = ["glslang_validator_path", "glslang_path"];
            let configured = settings
                .initialization_options
                .as_ref()
                .and_then(|opts| extract_path_from_json(opts, &keys))
                .or_else(|| {
                    settings
                        .settings
                        .as_ref()
                        .and_then(|s| extract_path_from_json(s, &keys))
                });

            if let Some(path) = resolve_configured_path(configured, worktree) {
                return Ok(path);
            }
        }

        // 1) Check PATH (Windows, Linux, macOS) for either glslangValidator or glslang
        if let Some(path) = worktree.which("glslangValidator").or_else(|| worktree.which("glslang")) {
            return Ok(path);
        }

        // 2) Check cached binary
        if let Some(path) = &self.cached_glslang
            && fs::metadata(path).is_ok_and(|s| s.is_file())
        {
            return Ok(path.clone());
        }

        let (platform, arch) = zed::current_platform();
        let exe = if matches!(platform, zed::Os::Windows) {
            ".exe"
        } else {
            ""
        };

        // 3) Check VULKAN_SDK environment variable if present
        if let Ok(vk_sdk) = std::env::var("VULKAN_SDK") {
            let candidate1 = format!("{vk_sdk}/bin/glslangValidator{exe}");
            let candidate2 = format!("{vk_sdk}/bin/glslang{exe}");
            if fs::metadata(&candidate1).is_ok_and(|s| s.is_file()) {
                self.cached_glslang = Some(candidate1.clone());
                return Ok(candidate1);
            }
            if fs::metadata(&candidate2).is_ok_and(|s| s.is_file()) {
                self.cached_glslang = Some(candidate2.clone());
                return Ok(candidate2);
            }
        }

        // 4) Common fallback locations per platform
        let fallbacks: &[&str] = match platform {
            zed::Os::Windows => &[
                "C:\\msys64\\ucrt64\\bin\\glslangValidator.exe",
                "C:\\msys64\\ucrt64\\bin\\glslang.exe",
                "C:\\msys64\\mingw64\\bin\\glslangValidator.exe",
                "C:\\msys64\\mingw64\\bin\\glslang.exe",
                "C:\\msys64\\clang64\\bin\\glslangValidator.exe",
                "C:\\msys64\\clang64\\bin\\glslang.exe",
                "C:\\Program Files\\glslang\\bin\\glslangValidator.exe",
                "C:\\Program Files\\glslang\\bin\\glslang.exe",
            ],
            zed::Os::Linux | zed::Os::Mac => &[
                "/usr/local/bin/glslangValidator",
                "/usr/local/bin/glslang",
                "/usr/bin/glslangValidator",
                "/usr/bin/glslang",
                "/opt/homebrew/bin/glslangValidator",
                "/opt/homebrew/bin/glslang",
            ],
        };
        for fallback in fallbacks {
            if fs::metadata(fallback).is_ok_and(|s| s.is_file()) {
                self.cached_glslang = Some(fallback.to_string());
                return Ok(fallback.to_string());
            }
        }

        // 5) Download official release from KhronosGroup/glslang GitHub Releases
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "KhronosGroup/glslang",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset = release
            .assets
            .iter()
            .find(|a| {
                let name = &a.name;
                // Exclude debug packages (80-250MB) and prioritize compact release builds (7-13MB)
                if !name.contains("release") {
                    return false;
                }
                match (platform, arch) {
                    (zed::Os::Windows, zed::Architecture::X8664 | zed::Architecture::X86) => {
                        name.contains("windows-x86_64")
                    }
                    (zed::Os::Windows, zed::Architecture::Aarch64) => {
                        name.contains("windows-arm64") || name.contains("windows-x86_64")
                    }
                    (zed::Os::Linux, zed::Architecture::X8664) => {
                        name.contains("linux-x86_64")
                    }
                    (zed::Os::Linux, zed::Architecture::Aarch64) => {
                        name.contains("linux-arm64") || name.contains("linux-aarch64") || name.contains("linux-x86_64")
                    }
                    (zed::Os::Mac, _) => {
                        name.contains("macos-universal") || name.contains("macos")
                    }
                    _ => false,
                }
            })
            .ok_or_else(|| {
                format!(
                    "Compatible glslang release asset not found for {platform:?}-{arch:?} in KhronosGroup/glslang release {}",
                    release.version
                )
            })?;

        let version_dir = format!("glslang-{}", release.version);
        let candidate_bin_validator = format!("{version_dir}/bin/glslangValidator{exe}");
        let candidate_bin_glslang = format!("{version_dir}/bin/glslang{exe}");
        let candidate_root_validator = format!("{version_dir}/glslangValidator{exe}");
        let candidate_root_glslang = format!("{version_dir}/glslang{exe}");

        if !fs::metadata(&candidate_bin_validator).is_ok_and(|s| s.is_file())
            && !fs::metadata(&candidate_bin_glslang).is_ok_and(|s| s.is_file())
            && !fs::metadata(&candidate_root_validator).is_ok_and(|s| s.is_file())
            && !fs::metadata(&candidate_root_glslang).is_ok_and(|s| s.is_file())
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let file_type = if asset.name.ends_with(".zip") {
                zed::DownloadedFileType::Zip
            } else if asset.name.ends_with(".tar.gz") || asset.name.ends_with(".tgz") {
                zed::DownloadedFileType::GzipTar
            } else {
                zed::DownloadedFileType::Zip
            };

            zed::download_file(&asset.download_url, &version_dir, file_type)
                .map_err(|e| format!("Failed to download glslang from KhronosGroup: {e}"))?;
        }

        let binary_path = if fs::metadata(&candidate_bin_validator).is_ok_and(|s| s.is_file()) {
            candidate_bin_validator
        } else if fs::metadata(&candidate_bin_glslang).is_ok_and(|s| s.is_file()) {
            candidate_bin_glslang
        } else if fs::metadata(&candidate_root_validator).is_ok_and(|s| s.is_file()) {
            candidate_root_validator
        } else if fs::metadata(&candidate_root_glslang).is_ok_and(|s| s.is_file()) {
            candidate_root_glslang
        } else {
            return Err(format!(
                "glslang executable not found in '{version_dir}/bin' or root of archive"
            ));
        };

        let _ = zed::make_file_executable(&binary_path);

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        self.cached_glslang = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for GlslExtendedExtension {
    fn new() -> Self {
        Self {
            cached_glsl_analyzer: None,
            cached_glsl_validator: None,
            cached_glslang: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            "glsl_validator" => {
                let validator = self.find_glsl_validator(language_server_id, worktree)?;
                let mut env = Vec::new();
                if let Ok(glslang) = self.find_glslang(language_server_id, worktree) {
                    env.push(("GLSLANG_VALIDATOR_PATH".to_string(), glslang));
                }
                if let Ok(analyzer) = self.find_glsl_analyzer(language_server_id, worktree) {
                    env.push(("GLSL_ANALYZER_PATH".to_string(), analyzer));
                }
                Ok(zed::Command {
                    command: validator,
                    args: vec![],
                    env,
                })
            }
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

        if server_name == "glsl_validator" {
            let mut opts = settings.initialization_options.unwrap_or_else(|| serde_json::json!({}));
            let is_empty_val = |v: Option<&serde_json::Value>| match v {
                None | Some(serde_json::Value::Null) => true,
                Some(serde_json::Value::String(s)) => s.trim().is_empty(),
                _ => false,
            };
            if is_empty_val(opts.get("glslang_validator_path"))
                && is_empty_val(opts.get("glslang_path"))
                && let Ok(glslang) = self.find_glslang(language_server_id, worktree)
            {
                opts["glslang_validator_path"] = serde_json::Value::String(glslang);
            }
            if is_empty_val(opts.get("glsl_analyzer_path"))
                && is_empty_val(opts.get("analyzer_path"))
                && let Ok(analyzer) = self.find_glsl_analyzer(language_server_id, worktree)
            {
                opts["glsl_analyzer_path"] = serde_json::Value::String(analyzer);
            }
            return Ok(Some(opts));
        }

        if let Some(opts) = settings.initialization_options {
            return Ok(Some(opts));
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
