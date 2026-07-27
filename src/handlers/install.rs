use std::process::Command;
use std::sync::Arc;

use uncver_artifacts::output;
use uncver_artifacts::podman::{Podman, TraefikOrchestrator};
use uncver_artifacts::ArtifactManager;

fn ensure_gui_build_deps() {
    let libs = ["pkg-config", "libgtk-3-dev", "libwebkit2gtk-4.1-dev", "libxdo-dev"];
    for lib in &libs {
        let check = Command::new("dpkg").args(["-s", lib]).output();
        match check {
            Ok(status) if status.status.success() => {}
            _ => {
                let _ = Command::new("sudo").args(["apt", "install", "-y", lib]).output();
            }
        }
    }
}

pub async fn install(podman: &Podman, artifacts: &Arc<ArtifactManager>, verbose: bool) -> anyhow::Result<()> {
    output::separator();
    output::header("Welcome to uncver-artifacts");
    output::bold("uncver — uncover the future");
    output::dim("A peer-to-peer artifact management ecosystem for autonomous AI agents.");
    output::separator();
    output::info("About us");
    output::dim("uncver is an open infrastructure for AI-native service discovery, streaming,");
    output::dim("and orchestration — built on Redis streams, Podman containers, and a shared");
    output::dim("artifact registry. Every component is a self-contained artifact that can be");
    output::dim("discovered, composed, and orchestrated across the network.");
    output::separator();
    output::info("This install will:");
    output::bullet("Sync the uncver artifact library", "git pull uncoverthefuture-org/uncver-artifact-lib");
    output::bullet("Build and start required artifacts", "containers on uncver-network");
    output::bullet("Register the CLI binary", "uncver-artifacts in PATH");
    output::separator();
    ensure_gui_build_deps();
    podman.ensure_installed()?;
    if !podman.is_available() {
        anyhow::bail!("Podman is required but could not be installed. Please install it manually: sudo apt-get install podman");
    }
    TraefikOrchestrator::ensure_network()?;
    TraefikOrchestrator::ensure_traefik()?;
    let lib_dir = crate::install::repo::sync_artifact_lib()?;
    crate::install::ensure_required_artifacts(&lib_dir, artifacts, verbose)?;
    crate::install::path::register_binary_in_path()?;
    output::header("✅ uncver-artifacts is ready!");
    output::bold("Running:");
    output::bullet("uncver-redis-stream", "Redis 7 on uncver-network");
    output::bullet("uncver-ollama", "Ollama LLM server (qwen3:0.6b)");
    output::bullet("uncver-ollama-qwen3-0.6b-ai", "AI router on Redis stream");
    output::bullet("uncver-traefik", "Reverse proxy on port 42080");
    output::bold("Quick commands:");
    output::cmd("uncver-artifacts ps", "List all running containers");
    output::cmd("uncver-artifacts logs <name>", "View container logs");
    output::cmd("uncver-artifacts list", "List all artifacts");
    output::cmd("uncver-artifacts refresh", "Pull updates & restart");
    output::dim("────────────────────────────────────────────");
    Ok(())
}

pub async fn refresh(podman: &Podman, artifacts: &Arc<ArtifactManager>, verbose: bool) -> anyhow::Result<()> {
    podman.ensure_installed()?;
    podman.ensure_machine_running()?;
    TraefikOrchestrator::ensure_network()?;
    TraefikOrchestrator::ensure_traefik()?;
    let lib_dir = crate::install::repo::sync_artifact_lib()?;
    crate::install::ensure_required_artifacts(&lib_dir, artifacts, verbose)?;
    output::success("All artifacts are up to date.");
    Ok(())
}
