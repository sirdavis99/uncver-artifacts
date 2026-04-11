use anyhow::Context;
use std::process::Command;

pub struct PodmanMachine;

impl PodmanMachine {
    pub fn new() -> Self {
        Self
    }

    pub fn is_running(&self) -> anyhow::Result<bool> {
        let output = Command::new("podman")
            .args(["machine", "list"])
            .output()
            .context("Failed to list podman machines")?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        if !output.status.success() {
            return Ok(false);
        }

        for line in stdout.lines() {
            let lower = line.to_lowercase();
            // Different podman versions render "Running" or "Currently running" differently
            if lower.contains("podman-machine")
                && (lower.contains("running") || lower.contains("up"))
            {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn start(&self) -> anyhow::Result<()> {
        tracing::info!("Starting Podman machine...");

        let output = Command::new("podman")
            .args(["machine", "start"])
            .output()
            .context("Failed to start podman machine")?;

        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        // If it failed, but the error says "already running", we consider it a success.
        if !output.status.success()
            && !stderr.to_lowercase().contains("already running")
            && !stdout.to_lowercase().contains("already running")
        {
            let init_output = Command::new("podman")
                .args(["machine", "init"])
                .output()
                .context("Failed to init podman machine")?;

            if !init_output.status.success() {
                anyhow::bail!("Failed to start or init podman machine");
            }

            let start_status = Command::new("podman")
                .args(["machine", "start"])
                .status()
                .context("Failed to start podman machine after init")?;

            if !start_status.success() {
                anyhow::bail!("Podman machine start failed after init");
            }
        }

        tracing::info!("Podman machine started successfully");
        Ok(())
    }

    pub fn stop(&self) -> anyhow::Result<()> {
        tracing::info!("Stopping Podman machine...");

        let status = Command::new("podman")
            .args(["machine", "stop"])
            .status()
            .context("Failed to stop podman machine")?;

        if !status.success() {
            tracing::warn!("Podman machine stop returned non-zero status, but continuing...");
        }

        Ok(())
    }

    pub fn info(&self) -> anyhow::Result<Option<String>> {
        let output = Command::new("podman")
            .args(["machine", "list"])
            .output()
            .context("Failed to get podman machine list")?;

        if output.status.success() {
            Ok(Some(String::from_utf8_lossy(&output.stdout).to_string()))
        } else {
            Ok(None)
        }
    }
}

impl Default for PodmanMachine {
    fn default() -> Self {
        Self::new()
    }
}
