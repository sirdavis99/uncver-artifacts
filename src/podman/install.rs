use anyhow::Context;
use std::process::Command;

pub struct PodmanInstaller;

impl PodmanInstaller {
    pub fn new() -> Self {
        Self
    }

    pub fn is_installed(&self) -> anyhow::Result<bool> {
        match Command::new("podman").arg("--version").output() {
            Ok(output) => Ok(output.status.success()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e).context("Failed to check podman installation"),
        }
    }

    pub fn version(&self) -> anyhow::Result<Option<String>> {
        match Command::new("podman").arg("--version").output() {
            Ok(output) if output.status.success() => Ok(Some(
                String::from_utf8_lossy(&output.stdout).trim().to_string(),
            )),
            Ok(_) => Ok(None),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e).context("Failed to get podman version"),
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

        let pkg_path_clone = pkg_path.clone();
        let download_result = std::thread::spawn(move || -> anyhow::Result<()> {
            let client = reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(300))
                .build()?;

            let mut response = client
                .get("https://github.com/containers/podman/releases/latest/download/podman-installer-macos-amd64.pkg")
                .send()
                .context("Failed to download Podman")?;

            let mut file = std::fs::File::create(&pkg_path_clone)?;
            std::io::copy(&mut response, &mut file)?;

            Ok(())
        })
        .join()
        .map_err(|e| anyhow::anyhow!("Thread panicked during download: {:?}", e))?;

        download_result?;

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

    pub fn enable_autostart(&self) -> anyhow::Result<()> {
        tracing::info!("Setting up Podman auto-start on system boot...");

        #[cfg(target_os = "macos")]
        {
            self.enable_autostart_macos()?;
        }

        #[cfg(target_os = "linux")]
        {
            self.enable_autostart_linux()?;
        }

        #[cfg(target_os = "windows")]
        {
            self.enable_autostart_windows()?;
        }

        tracing::info!("Podman auto-start enabled");
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn enable_autostart_macos(&self) -> anyhow::Result<()> {
        let launch_agents_dir = dirs::home_dir()
            .map(|h| h.join("Library/LaunchAgents"))
            .context("Failed to get home directory")?;

        std::fs::create_dir_all(&launch_agents_dir)?;

        let plist_path = launch_agents_dir.join("com.uncver.podman.plist");
        let plist_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.uncver.podman</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/podman</string>
        <string>machine</string>
        <string>start</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <false/>
</dict>
</plist>
"#;

        std::fs::write(&plist_path, plist_content)?;
        tracing::info!("Created LaunchAgent at {:?}", plist_path);

        let _ = Command::new("launchctl")
            .args(["load", plist_path.to_str().unwrap()])
            .output();

        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn enable_autostart_linux(&self) -> anyhow::Result<()> {
        let systemd_dir = std::env::var("XDG_CONFIG_HOME")
            .map(|p| std::path::PathBuf::from(p).join("systemd/user"))
            .or_else(|_| dirs::home_dir().map(|h| h.join(".config/systemd/user")))
            .context("Failed to determine systemd user directory")?;

        std::fs::create_dir_all(&systemd_dir)?;

        let service_path = systemd_dir.join("podman-machines-start.service");
        let service_content = r#"[Unit]
Description=Podman machines start
[Service]
Type=oneshot
ExecStart=/usr/bin/podman machine start -a
StandardOutput=journal
[Install]
WantedBy=default.target
"#;

        std::fs::write(&service_path, service_content)?;
        tracing::info!("Created systemd service at {:?}", service_path);

        let _ = Command::new("systemctl")
            .args(["--user", "enable", "podman-machines-start.service"])
            .output();

        let _ = Command::new("systemctl")
            .args(["--user", "start", "podman-machines-start.service"])
            .output();

        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn enable_autostart_windows(&self) -> anyhow::Result<()> {
        let task_name = "uncver_podman_start";

        let exe_path = std::env::current_exe()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "podman".to_string());

        let status = Command::new("schtasks")
            .args([
                "/Create",
                "/SC",
                "ONLOGIN",
                "/TN",
                task_name,
                "/TR",
                &format!("{} machine start", exe_path),
                "/F",
            ])
            .status()
            .context("Failed to create scheduled task")?;

        if !status.success() {
            anyhow::bail!("schtasks creation failed");
        }

        tracing::info!("Created Windows Task Scheduler task: {}", task_name);
        Ok(())
    }

    pub fn disable_autostart(&self) -> anyhow::Result<()> {
        tracing::info!("Removing Podman auto-start...");

        #[cfg(target_os = "macos")]
        {
            let plist_path =
                dirs::home_dir().map(|h| h.join("Library/LaunchAgents/com.uncver.podman.plist"));
            if let Some(path) = plist_path {
                if path.exists() {
                    let _ = Command::new("launchctl")
                        .args(["unload", path.to_str().unwrap()])
                        .output();
                    let _ = std::fs::remove_file(path);
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            let _ = Command::new("systemctl")
                .args(["--user", "disable", "podman-machines-start.service"])
                .output();
            if let Some(config_dir) = dirs::config_dir() {
                let service_path = config_dir.join("systemd/user/podman-machines-start.service");
                let _ = std::fs::remove_file(service_path);
            }
        }

        #[cfg(target_os = "windows")]
        {
            let _ = Command::new("schtasks")
                .args(["/Delete", "/TN", "uncver_podman_start", "/F"])
                .output();
        }

        tracing::info!("Podman auto-start disabled");
        Ok(())
    }
}

impl Default for PodmanInstaller {
    fn default() -> Self {
        Self::new()
    }
}
