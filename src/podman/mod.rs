pub mod install;
pub mod machine;
pub mod runner;

pub use install::PodmanInstaller;
pub use machine::PodmanMachine;
pub use runner::PodmanRunner;

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

pub struct Podman {
    installer: PodmanInstaller,
    machine: PodmanMachine,
    runner: PodmanRunner,
}

impl Podman {
    pub fn new() -> Self {
        Self {
            installer: PodmanInstaller::new(),
            machine: PodmanMachine::new(),
            runner: PodmanRunner::new(),
        }
    }

    pub fn ensure_installed(&self) -> Result<()> {
        if !self.installer.is_installed()? {
            tracing::info!("Podman not found, initiating installation...");
            self.installer.install()?;
        }
        Ok(())
    }

    pub fn ensure_machine_running(&self) -> Result<()> {
        if !self.machine.is_running()? {
            tracing::info!("Podman machine not running, starting...");
            self.machine.start()?;
        }
        Ok(())
    }

    pub fn run(&self, image: &str) -> Result<String> {
        self.runner.run(image)
    }

    pub fn is_available(&self) -> bool {
        self.installer.is_installed().unwrap_or(false)
    }

    pub fn is_machine_running(&self) -> bool {
        self.machine.is_running().unwrap_or(false)
    }
}

impl Default for Podman {
    fn default() -> Self {
        Self::new()
    }
}
