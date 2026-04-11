//! Self-upgrade functionality for uncver-artifacts
//!
//! Handles checking for new versions and upgrading the binary

use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

const GITHUB_REPO: &str = "sirdavis99/uncver-artifacts";
const GITHUB_API_URL: &str = "https://api.github.com/repos";

#[derive(Debug, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: String,
    pub body: Option<String>,
    pub assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
}

pub struct UpgradeManager;

impl Default for UpgradeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UpgradeManager {
    pub fn new() -> Self {
        Self
    }

    /// Get the current version of the binary
    pub fn current_version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// Check for the latest version from GitHub releases
    pub async fn check_latest_version() -> Result<Option<GitHubRelease>> {
        let url = format!("{}/{}/releases/latest", GITHUB_API_URL, GITHUB_REPO);

        debug!("Fetching latest release from: {}", url);

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", "uncver-artifacts-upgrade")
            .send()
            .await
            .context("Failed to fetch latest release")?;

        if !response.status().is_success() {
            anyhow::bail!("GitHub API returned error: {}", response.status());
        }

        let release: GitHubRelease = response
            .json()
            .await
            .context("Failed to parse GitHub release")?;

        Ok(Some(release))
    }

    /// Compare versions and determine if upgrade is needed
    pub fn needs_upgrade(current: &str, latest: &str) -> bool {
        // Remove 'v' prefix if present
        let current = current.trim_start_matches('v');
        let latest = latest.trim_start_matches('v');

        let c_parts: Vec<&str> = current.split('.').collect();
        let l_parts: Vec<&str> = latest.split('.').collect();

        for (c, l) in c_parts.iter().zip(l_parts.iter()) {
            let c_num: u32 = c.parse().unwrap_or(0);
            let l_num: u32 = l.parse().unwrap_or(0);

            if l_num > c_num {
                return true;
            } else if c_num > l_num {
                return false;
            }
        }

        l_parts.len() > c_parts.len()
    }

    /// Get the appropriate asset for the current platform
    pub fn get_platform_asset(release: &GitHubRelease) -> Option<&GitHubAsset> {
        let target = Self::get_target_triple();

        release
            .assets
            .iter()
            .find(|asset| asset.name.contains(&target))
    }

    fn get_target_triple() -> String {
        // Determine target triple based on platform
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        return "x86_64-apple-darwin".to_string();

        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        return "aarch64-apple-darwin".to_string();

        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        return "x86_64-unknown-linux-gnu".to_string();

        #[cfg(not(any(
            all(target_os = "macos", target_arch = "x86_64"),
            all(target_os = "macos", target_arch = "aarch64"),
            all(target_os = "linux", target_arch = "x86_64")
        )))]
        return "unknown".to_string();
    }

    /// Download and install the latest version
    pub async fn upgrade_to_version(&self, asset: &GitHubAsset) -> Result<PathBuf> {
        info!("Downloading {}...", asset.name);

        let client = reqwest::Client::new();
        let response = client
            .get(&asset.browser_download_url)
            .send()
            .await
            .context("Failed to download update")?;

        if !response.status().is_success() {
            anyhow::bail!("Download failed: {}", response.status());
        }

        let bytes = response.bytes().await.context("Failed to read download")?;

        // Create temp directory for extraction
        let temp_dir = tempfile::tempdir().context("Failed to create temp directory")?;
        let archive_path = temp_dir.path().join(&asset.name);

        fs::write(&archive_path, bytes).context("Failed to write archive")?;

        // Extract the binary
        let extracted_binary = Self::extract_binary(&archive_path, temp_dir.path())?;

        // Get current binary path
        let current_exe = env::current_exe().context("Failed to get current executable path")?;

        // Backup current binary
        let backup_path = current_exe.with_extension("backup");
        fs::copy(&current_exe, &backup_path).context("Failed to backup current binary")?;

        // Replace current binary
        fs::copy(&extracted_binary, &current_exe).context("Failed to replace binary")?;

        // Make executable
        #[cfg(unix)]
        {
            let mut perms = fs::metadata(&current_exe)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&current_exe, perms).context("Failed to set permissions")?;
        }

        // Clean up backup on success
        let _ = fs::remove_file(&backup_path);

        info!("Upgrade complete!");
        Ok(current_exe)
    }

    fn extract_binary(archive_path: &Path, extract_dir: &std::path::Path) -> Result<PathBuf> {
        let extension = archive_path.extension().and_then(|e| e.to_str());

        match extension {
            Some("gz") | Some("tgz") => {
                // tar.gz archive
                let output = std::process::Command::new("tar")
                    .args([
                        "-xzf",
                        archive_path.to_str().unwrap(),
                        "-C",
                        extract_dir.to_str().unwrap(),
                    ])
                    .output()
                    .context("Failed to extract archive")?;

                if !output.status.success() {
                    anyhow::bail!(
                        "Extraction failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Some("zip") => {
                // Zip archive
                let output = std::process::Command::new("unzip")
                    .args([
                        "-o",
                        archive_path.to_str().unwrap(),
                        "-d",
                        extract_dir.to_str().unwrap(),
                    ])
                    .output()
                    .context("Failed to extract archive")?;

                if !output.status.success() {
                    anyhow::bail!(
                        "Extraction failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            _ => {
                // Assume it's the binary itself
                return Ok(archive_path.to_path_buf());
            }
        }

        // Find the binary in extracted contents
        let binary_name = "uncver-artifacts";
        for entry in fs::read_dir(extract_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.file_stem().is_some_and(|s| s == binary_name) {
                return Ok(path);
            }
        }

        anyhow::bail!("Binary not found in archive")
    }
}

/// Check for updates and upgrade if available
pub async fn check_and_upgrade(force: bool) -> Result<String> {
    let manager = UpgradeManager::new();
    let current_version = UpgradeManager::current_version();

    println!("Current version: {}", current_version);
    println!("Checking for updates...");

    let release = match UpgradeManager::check_latest_version().await? {
        Some(r) => r,
        None => return Ok("No releases found".to_string()),
    };

    let latest_version = release.tag_name.clone();
    println!("Latest version: {}", latest_version);

    let needs_upgrade = force || UpgradeManager::needs_upgrade(&current_version, &latest_version);

    if !needs_upgrade {
        return Ok(format!(
            "Already on the latest version ({}). Use --force to reinstall.",
            current_version
        ));
    }

    // Find appropriate asset
    let asset = match UpgradeManager::get_platform_asset(&release) {
        Some(a) => a,
        None => {
            return Ok(format!(
                "No pre-built binary available for your platform.\n\
                 Current platform: {}\n\
                 You can build from source:\n\
                 cargo install --git https://github.com/{}",
                UpgradeManager::get_target_triple(),
                GITHUB_REPO
            ));
        }
    };

    println!("Found update: {}", release.name);
    if let Some(body) = &release.body {
        println!("\nRelease notes:\n{}", body);
    }

    println!("\nDownloading from: {}", asset.browser_download_url);

    // Perform upgrade
    let new_binary = manager.upgrade_to_version(asset).await?;

    Ok(format!(
        "✓ Successfully upgraded to {}\n\
         Binary location: {}",
        latest_version,
        new_binary.display()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        assert!(UpgradeManager::needs_upgrade("0.1.0", "0.2.0"));
        assert!(UpgradeManager::needs_upgrade("0.1.0", "0.1.1"));
        assert!(!UpgradeManager::needs_upgrade("0.1.0", "0.1.0"));
        assert!(!UpgradeManager::needs_upgrade("0.2.0", "0.1.0")); // Don't downgrade
        assert!(UpgradeManager::needs_upgrade("0.1.0", "v0.2.0")); // Handle v prefix
    }
}
