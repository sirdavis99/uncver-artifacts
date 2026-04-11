use anyhow::Context;
use std::process::Command;

pub struct PodmanInstaller;

impl PodmanInstaller {
    pub fn new() -> Self {
        Self
    }

    pub fn is_installed(&self) -> anyhow::Result<bool> {
        let output = Command::new("podman")
            .arg("--version")
            .output()
            .context("Failed to check podman installation")?;

        Ok(output.status.success())
    }

    pub fn version(&self) -> anyhow::Result<Option<String>> {
        let output = Command::new("podman")
            .arg("--version")
            .output()
            .context("Failed to get podman version")?;

        if output.status.success() {
            Ok(Some(
                String::from_utf8_lossy(&output.stdout).trim().to_string(),
            ))
        } else {
            Ok(None)
        }
    }

    pub fn install(&self) -> anyhow::Result<()> {
        tracing::info!("Starting Podman installation...");

        #[cfg(target_os = "macos")]
        {
            self.install_macos()?;
        }

        #[cfg(target_os = "linux")]
        {
            self.install_linux()?;
        }

        #[cfg(target_os = "windows")]
        {
            self.install_windows()?;
        }

        tracing::info!("Podman installation complete");
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn install_macos(&self) -> anyhow::Result<()> {
        let has_brew = Command::new("brew")
            .args(["info", "podman"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if has_brew {
            tracing::info!("Installing Podman via Homebrew...");
            let status = Command::new("brew")
                .args(["install", "podman"])
                .status()
                .context("Failed to run brew install")?;

            if !status.success() {
                anyhow::bail!("Homebrew installation failed");
            }
        } else {
            tracing::info!("Homebrew not found, attempting direct download...");
            self.install_podman_macOS_download()?;
        }

        self.init_machine()?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    #[allow(non_snake_case)]
    fn install_podman_macOS_download(&self) -> anyhow::Result<()> {
        use std::path::PathBuf;

        let download_url = "https://github.com/containers/podman/releases/latest/download/podman-installer-macos-amd64.pkg";
        let temp_dir = std::env::temp_dir();
        let pkg_path: PathBuf = temp_dir.join("podman-installer.pkg");

        tracing::info!("Downloading Podman from {}", download_url);

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()?;

        let mut response = client
            .get(download_url)
            .send()
            .context("Failed to download Podman")?;

        let mut file = std::fs::File::create(&pkg_path)?;
        std::io::copy(&mut response, &mut file)?;

        tracing::info!("Installing Podman from {}", pkg_path.display());

        let status = Command::new("sudo")
            .args([
                "installer",
                "-pkg",
                pkg_path.to_str().unwrap(),
                "-target",
                "/",
            ])
            .status()
            .context("Failed to install Podman package")?;

        if !status.success() {
            anyhow::bail!("Podman installation package failed");
        }

        let _ = std::fs::remove_file(pkg_path);
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn install_linux(&self) -> anyhow::Result<()> {
        let os_release =
            std::fs::read_to_string("/etc/os-release").context("Failed to read os-release")?;

        if os_release.contains("ID=ubuntu") || os_release.contains("ID=debian") {
            tracing::info!("Installing Podman via apt...");
            Command::new("sudo")
                .args(["apt-get", "update"])
                .status()
                .context("Failed to apt-get update")?;

            Command::new("sudo")
                .args(["apt-get", "install", "-y", "podman"])
                .status()
                .context("Failed to apt-get install podman")?;
        } else if os_release.contains("ID=fedora")
            || os_release.contains("ID=centos")
            || os_release.contains("ID=rhel")
        {
            tracing::info!("Installing Podman via dnf...");
            Command::new("sudo")
                .args(["dnf", "install", "-y", "podman"])
                .status()
                .context("Failed to dnf install podman")?;
        } else {
            tracing::info!("Using podman static binary or script...");
            self.install_via_script()?;
        }

        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn install_windows(&self) -> anyhow::Result<()> {
        let has_winget = Command::new("winget")
            .args(["list", "Podman"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if has_winget {
            tracing::info!("Installing Podman via winget...");
            let status = Command::new("winget")
                .args([
                    "install",
                    "--id",
                    "RedHat.Podman",
                    "--accept-package-agreements",
                    "--accept-source-agreements",
                ])
                .status()
                .context("Failed to winget install podman")?;

            if !status.success() {
                anyhow::bail!("Winget installation failed");
            }
        } else {
            self.install_via_script()?;
        }

        self.init_machine()?;
        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    fn install_via_script(&self) -> anyhow::Result<()> {
        let script_url = "https://get.podman.io";

        tracing::info!("Installing Podman via official script...");

        let status = Command::new("sh")
            .arg("-c")
            .arg(format!("curl -SL {} | sh", script_url))
            .status()
            .context("Failed to run Podman install script")?;

        if !status.success() {
            anyhow::bail!("Podman install script failed");
        }

        Ok(())
    }

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    fn init_machine(&self) -> anyhow::Result<()> {
        let output = Command::new("podman")
            .args(["machine", "list"])
            .output()
            .context("Failed to list podman machines")?;

        if !String::from_utf8_lossy(&output.stdout).contains("podman-machine-default") {
            tracing::info!("Initializing Podman machine...");
            let status = Command::new("podman")
                .args(["machine", "init"])
                .status()
                .context("Failed to podman machine init")?;

            if !status.success() {
                anyhow::bail!("Podman machine init failed");
            }
        }

        Ok(())
    }
}

impl Default for PodmanInstaller {
    fn default() -> Self {
        Self::new()
    }
}
