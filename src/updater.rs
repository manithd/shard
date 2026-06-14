//! Self-update via GitHub Releases using `release-hub`.

use release_hub::{Config, GitHubSource, UpdaterBuilder};

/// Information about an available update.
#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub version: String,
}

/// Check GitHub for a newer release. Returns `Ok(Some(info))` if an update
/// is available, `Ok(None)` if up to date, `Err` on network/parse failure.
/// This is non-fatal — never block app startup on it.
pub fn check_for_update() -> Result<Option<UpdateInfo>, anyhow::Error> {
    let _ = rustls::crypto::ring::default_provider().install_default();
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let current_version = env!("CARGO_PKG_VERSION");
        let source = GitHubSource::new("manithd", "PDF2WebP");
        let config = Config {
            endpoints: vec![],
            ..Default::default()
        };

        let updater = UpdaterBuilder::new("PDF2WebP", current_version, config)
            .source(Box::new(source))
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        match updater.check().await? {
            Some(update) => Ok(Some(UpdateInfo {
                version: update.version.to_string(),
            })),
            None => Ok(None),
        }
    })
}

/// Download and install the update, then relaunch the app.
/// Should only be called after the user confirms and no conversion is active.
pub fn download_and_install() -> Result<(), anyhow::Error> {
    let _ = rustls::crypto::ring::default_provider().install_default();
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let current_version = env!("CARGO_PKG_VERSION");
        let source = GitHubSource::new("manithd", "PDF2WebP");
        let config = Config {
            endpoints: vec![],
            ..Default::default()
        };

        let updater = UpdaterBuilder::new("PDF2WebP", current_version, config)
            .source(Box::new(source))
            .timeout(std::time::Duration::from_secs(120))
            .build()?;

        // Check and get the matching update for this platform.
        if let Some(update) = updater.check().await? {
            updater.download_and_install(&update, |_| {}).await?;
            updater.relaunch()?;
        }

        Ok(())
    })
}
