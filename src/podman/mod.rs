pub mod install;
pub mod machine;
pub mod runner;
pub mod traefik;

pub use install::PodmanInstaller;
pub use machine::PodmanMachine;
pub use runner::PodmanRunner;
pub use traefik::TraefikOrchestrator;

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PodmanError {
    #[error("Podman is not installed")]
    NotInstalled,
    #[error("Podman machine failed: {0}")]
    MachineError(String),
    #[error("Podman run failed: {0}")]
    RunError(String),
    #[error("Installation failed: {0}")]
    InstallError(String),
}

use std::sync::Arc;

pub struct Podman {
    pub(crate) inner: Arc<PodmanInner>,
}

pub struct PodmanInner {
    installer: PodmanInstaller,
    machine: PodmanMachine,
    runner: PodmanRunner,
}

impl Podman {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(PodmanInner {
                installer: PodmanInstaller::new(),
                machine: PodmanMachine::new(),
                runner: PodmanRunner::new(),
            }),
        }
    }
}

impl Clone for Podman {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl Podman {
    pub fn ensure_installed(&self) -> Result<()> {
        if !self.inner.installer.is_installed()? {
            tracing::info!("Podman not found, initiating installation...");
            self.inner.installer.install()?;
        }
        Ok(())
    }

    pub fn ensure_machine_running(&self) -> Result<()> {
        if !self.inner.machine.is_running()? {
            tracing::info!("Podman machine not running, starting...");
            self.inner.machine.start()?;
        }
        Ok(())
    }

    pub fn run(&self, image: &str) -> Result<String> {
        self.inner.runner.run(image)
    }

    pub fn run_detached(
        &self,
        image: &str,
        name: Option<&str>,
        ports: Option<&Vec<String>>,
        env: Option<&std::collections::HashMap<String, String>>,
    ) -> Result<String> {
        self.inner.runner.run_detached(image, name, ports, env)
    }

    pub fn build(&self, tag: &str, path: &str) -> Result<()> {
        self.inner.runner.build(tag, path)
    }

    pub fn list_containers(&self) -> Result<Vec<runner::ContainerInfo>> {
        self.inner.runner.list_containers()
    }

    pub fn get_logs(&self, name: &str) -> Result<String> {
        let output = std::process::Command::new("podman")
            .args(["logs", name])
            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string() + &String::from_utf8_lossy(&output.stderr))
    }

    pub fn is_available(&self) -> bool {
        self.inner.installer.is_installed().unwrap_or(false)
    }

    pub fn is_machine_running(&self) -> bool {
        self.inner.machine.is_running().unwrap_or(false)
    }

    pub fn get_socket_path() -> String {
        // 1. Try to find the socket from podman machine (macOS/Windows)
        let output = std::process::Command::new("podman")
            .args(["machine", "inspect", "--format", "{{(index .ConnectionInfo.PodmanSocket.Path)}}"])
            .output();

        if let Ok(out) = output {
            let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !path.is_empty() {
                return path;
            }
        }

        // 2. Fallback to common Linux paths
        if std::path::Path::new("/var/run/docker.sock").exists() {
            return "/var/run/docker.sock".to_string();
        }
        if std::path::Path::new("/run/podman/podman.sock").exists() {
            return "/run/podman/podman.sock".to_string();
        }

        // 3. Last resort default
        "/var/run/docker.sock".to_string()
    }
}

impl Default for Podman {
    fn default() -> Self {
        Self::new()
    }
}
