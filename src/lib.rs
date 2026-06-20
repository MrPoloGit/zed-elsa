use zed_extension_api::{
    self as zed, Architecture, DownloadedFileType, LanguageServerId,
    LanguageServerInstallationStatus, Os, Result, Worktree,
};

const LSP_VERSION: &str = "0.1.0";

struct ElsaExtension {
    cached_binary: Option<String>,
}

impl zed::Extension for ElsaExtension {
    fn new() -> Self {
        ElsaExtension {
            cached_binary: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<zed::Command> {
        let binary = self.lsp_binary(language_server_id)?;
        Ok(zed::Command {
            command: binary,
            args: vec![],
            env: vec![],
        })
    }
}

impl ElsaExtension {
    fn lsp_binary(&mut self, server_id: &LanguageServerId) -> Result<String> {
        let (os, arch) = zed::current_platform();

        let (asset, bin) = platform_asset(os, arch)?;

        let install_dir = format!("elsa-lsp-{}", LSP_VERSION);
        let binary_path = format!("{}/{}", install_dir, bin);

        // Skip download if we already have it.
        if self
            .cached_binary
            .as_deref()
            .map(|p| p == binary_path)
            .unwrap_or(false)
        {
            return Ok(binary_path);
        }

        zed::set_language_server_installation_status(
            server_id,
            &LanguageServerInstallationStatus::Downloading,
        );

        let release = zed::github_release_by_tag_name(
            "MrPoloGit/elsa-lsp",
            &format!("v{}", LSP_VERSION),
        )
        .map_err(|e| format!("failed to fetch elsa-lsp release: {e}"))?;

        let asset_info = release
            .assets
            .iter()
            .find(|a| a.name == asset)
            .ok_or_else(|| format!("no asset '{asset}' in elsa-lsp v{LSP_VERSION}"))?;

        let file_type = if asset.ends_with(".zip") {
            DownloadedFileType::Zip
        } else {
            DownloadedFileType::GzipTar
        };

        zed::download_file(&asset_info.download_url, &install_dir, file_type)
            .map_err(|e| format!("failed to download elsa-lsp: {e}"))?;

        zed::make_file_executable(&binary_path)
            .map_err(|e| format!("failed to mark elsa-lsp executable: {e}"))?;

        self.cached_binary = Some(binary_path.clone());
        Ok(binary_path)
    }
}

fn platform_asset(os: Os, arch: Architecture) -> Result<(String, String)> {
    let (os_str, arch_str, ext) = match (os, arch) {
        (Os::Mac, Architecture::Aarch64) => ("darwin", "aarch64", "tar.gz"),
        (Os::Linux, Architecture::Aarch64) => ("linux", "aarch64", "tar.gz"),
        (Os::Linux, Architecture::X8664) => ("linux", "x86_64", "tar.gz"),
        (Os::Windows, Architecture::Aarch64) => ("windows", "aarch64", "zip"),
        (Os::Windows, Architecture::X8664) => ("windows", "x86_64", "zip"),
        _ => return Err("elsa-lsp: unsupported platform".into()),
    };
    let bin = if os_str == "windows" {
        "elsa-lsp.exe".to_string()
    } else {
        "elsa-lsp".to_string()
    };
    Ok((format!("elsa-lsp-{}-{}.{}", os_str, arch_str, ext), bin))
}

zed::register_extension!(ElsaExtension);
