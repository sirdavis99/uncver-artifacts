use clap::{Parser, Subcommand};
use std::process::Command;
use std::sync::Arc;
use tracing::{error, info};

use uncver_artifacts::podman::{Podman, TraefikOrchestrator};
use uncver_artifacts::{open_gui_window, ArtifactConfig, ArtifactManager};

mod upgrade;

#[derive(Parser)]
#[command(name = "uncver-artifacts")]
#[command(about = "CLI tool for managing uncver artifacts with Podman integration")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install and setup required dependencies (Podman)
    Install,
    /// List all artifacts
    List,
    /// Start an artifact by name
    Start {
        /// Name of the artifact to start
        name: String,
    },
    /// Create a new artifact
    Create {
        /// Name of the artifact
        #[arg(short, long)]
        name: String,
        /// Description of the artifact
        #[arg(short, long)]
        description: Option<String>,
        /// URL for the artifact repository
        #[arg(short, long)]
        url: Option<String>,
        /// Local path to the artifact code
        #[arg(short, long)]
        local_path: Option<String>,
        /// Container image to use
        #[arg(short, long)]
        container_image: Option<String>,
    },
    /// Delete an artifact
    Delete {
        /// Name of the artifact to delete
        name: String,
    },
    /// Watch artifacts directory for changes
    Watch,
    /// Run the default artifacts
    Run,
    /// Upgrade to the latest version
    Upgrade {
        /// Force upgrade even if version is up to date
        #[arg(long)]
        force: bool,
    },
    /// List all podman containers
    Ps,
    /// View logs for a specific container
    Logs {
        /// Name of the container
        name: String,
    },
    /// Reset the environment (clears all uncver containers and restarts infrastructure)
    Reset,
    /// Enable auto-start of Podman machine on system boot
    Autostart {
        /// Disable auto-start instead of enabling
        #[arg(long)]
        disable: bool,
    },
    /// Start the tray application
    Tray,
    /// Load and start an artifact from a local directory
    Load {
        /// Path to the directory containing artifact.json
        path: String,
    },
    /// Internal command to launch a standalone native webview window
    Viewer {
        /// URL to open
        #[arg(value_name = "URL")]
        url: String,
        /// Width of the window
        #[arg(long)]
        width: Option<u16>,
        /// Height of the window
        #[arg(long)]
        height: Option<u16>,
        /// X position
        #[arg(long)]
        x: Option<i32>,
        /// Y position
        #[arg(long)]
        y: Option<i32>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("uncver_artifacts=debug,info")
        .init();

    info!("Starting uncver-artifacts CLI...");

    let cli = Cli::parse();
    let podman = Podman::new();
    let artifacts = Arc::new(ArtifactManager::new()?);

    match cli.command {
        Commands::Install => {
            info!("Installing dependencies...");
            podman.ensure_installed()?;
            podman.ensure_machine_running()?;
            podman.enable_autostart()?;
            info!("Installation complete!");
        }
        Commands::List => {
            let list = artifacts.list_artifacts().await?;
            if list.is_empty() {
                println!("No artifacts found.");
            } else {
                println!("Artifacts:");
                for artifact in list {
                    println!(
                        "  - {}: {}",
                        artifact.name,
                        artifact.description.as_deref().unwrap_or("No description")
                    );
                }
            }
        }
        Commands::Start { name } => {
            info!("Starting artifact: {}", name);
            let list = artifacts.list_artifacts().await?;
            if let Some(artifact) = list.iter().find(|a| a.name == name) {
                if let Some(ref image) = artifact.container_image {
                    podman.ensure_installed()?;
                    podman.ensure_machine_running()?;
                    match podman.run_detached(
                        image,
                        Some(&name),
                        artifact.ports.as_ref(),
                        artifact.environment.as_ref(),
                    ) {
                        Ok(output) => {
                            println!("Artifact started in background with ID:\n{}", output);
                            
                            // Register Traefik route
                            let port = artifact.gui_window.as_ref().and_then(|g| g.port).unwrap_or(8080);
                            let _ = TraefikOrchestrator::register_artifact_route(&artifact.name, port);
                            
                            open_gui_window(artifact);
                        }
                        Err(e) => error!("Failed to start artifact: {}", e),
                    }
                } else {
                    error!("Artifact '{}' has no container image specified", name);
                }
            } else {
                error!("Artifact '{}' not found", name);
            }
        }
        Commands::Create {
            name,
            description,
            url,
            local_path,
            container_image,
        } => {
            let config = ArtifactConfig {
                name,
                description,
                url,
                local_path,
                container_image,
                gui_window: None,
                ports: None,
                environment: None,
            };
            let path = artifacts.create_artifact(&config)?;
            info!("Created artifact at: {:?}", path);
        }
        Commands::Delete { name } => {
            artifacts.delete_artifact(&name)?;
            info!("Deleted artifact: {}", name);
        }
        Commands::Watch => {
            info!("Watching artifacts directory for changes...");
            info!("Press Ctrl+C to stop");

            use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};

            let (tx, rx) = std::sync::mpsc::channel();

            let path = uncver_artifacts::paths::get_artifacts_dir()?;

            let mut watcher = RecommendedWatcher::new(
                move |res: notify::Result<Event>| {
                    if let Ok(event) = res {
                        for path in event.paths {
                            if path.extension().is_some_and(|ext| ext == "json") {
                                let _ = tx.send(path);
                            }
                        }
                    }
                },
                notify::Config::default(),
            )?;

            watcher.watch(&path, RecursiveMode::Recursive)?;

            loop {
                match rx.recv() {
                    Ok(path) => info!("Artifact updated: {:?}", path),
                    Err(e) => {
                        error!("Watch error: {}", e);
                        break;
                    }
                }
            }
        }
        Commands::Run => {
            info!("Running default artifacts...");
            podman.ensure_installed()?;
            podman.ensure_machine_running()?;

            let list = artifacts.list_artifacts().await?;
            for artifact in list {
                if let Some(ref image) = artifact.container_image {
                    info!("Starting artifact: {}", artifact.name);
                    match podman.run_detached(
                        image,
                        Some(&artifact.name),
                        artifact.ports.as_ref(),
                        artifact.environment.as_ref(),
                    ) {
                        Ok(output) => {
                            println!(
                                "{} started in background with ID:\n{}",
                                artifact.name, output
                            );
                            open_gui_window(&artifact);
                        }
                        Err(e) => error!("Failed to start {}: {}", artifact.name, e),
                    }
                }
            }
        }
        Commands::Upgrade { force } => {
            info!("Checking for updates...");
            match upgrade::check_and_upgrade(force).await {
                Ok(msg) => println!("{}", msg),
                Err(e) => {
                    error!("Upgrade failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Tray => {
            uncver_artifacts::tray::run_tray()?;
        }
        Commands::Reset => {
            info!("Resetting uncver environment...");
            // 1. Kill everything with uncver- prefix
            let output = Command::new("podman")
                .args(["ps", "-a", "--format", "{{.Names}}"])
                .output()?;
            let names = String::from_utf8_lossy(&output.stdout);
            for name in names.lines() {
                if name.contains("uncver-") || name.contains("redis-stream-push-gui") {
                    info!("Removing container: {}", name);
                    let _ = Command::new("podman").args(["rm", "-f", name]).output();
                }
            }

            // 2. Clear stale Traefik routes from the global data dir
            if let Ok(config_dir) = uncver_artifacts::paths::get_traefik_config_dir() {
                info!("Clearing stale Traefik routes...");
                let _ = std::fs::remove_dir_all(&config_dir);
                let _ = std::fs::create_dir_all(&config_dir);
            }

            // 3. Ensure Network
            TraefikOrchestrator::ensure_network()?;

            // 4. Restart Base Infrastructure
            TraefikOrchestrator::ensure_traefik()?;

            // 4. Start Redis on bridge
            info!("Starting uncver-redis-stream on bridge...");
            let _ = Command::new("podman")
                .args([
                    "run", "-d",
                    "--name", "uncver-redis-stream",
                    "--network", "uncver-network",
                    "docker.io/library/redis:7-alpine"
                ])
                .output()?;
            
            info!("Reset complete! Traefik and Redis are running on 'uncver-network'.");
        }
        Commands::Autostart { disable } => {
            if disable {
                podman.disable_autostart()?;
                println!("Podman auto-start disabled.");
            } else {
                podman.enable_autostart()?;
                println!("Podman auto-start enabled. Podman machine will start on system boot.");
            }
        }
        Commands::Ps => {
            let containers = podman.list_containers()?;
            println!("{:<30} {:<30} {:<20} {:<20}", "ID", "NAME", "IMAGE", "STATE");
            for c in containers {
                println!("{:<30} {:<30} {:<20} {:<20}", c.id, c.name, c.image, c.state);
            }
        }
        Commands::Logs { name } => {
            let logs = podman.get_logs(&name)?;
            println!("{}", logs);
        }
        Commands::Load { path } => {
            let target_path = std::path::PathBuf::from(&path);
            let absolute_path = std::fs::canonicalize(&target_path)
                .map_err(|e| anyhow::anyhow!("Invalid path or directory not found: {}", e))?;

            // Ensure Base infrastructure
            TraefikOrchestrator::ensure_traefik()?;

            let json_path = absolute_path.join("artifact.json");
            if !json_path.exists() {
                anyhow::bail!("No artifact.json found in {:?}", absolute_path);
            }

            info!("Loading artifact config from {:?}", json_path);
            let content = std::fs::read_to_string(&json_path)?;
            let mut config: ArtifactConfig = serde_json::from_str(&content)
                .map_err(|e| anyhow::anyhow!("Invalid artifact.json format: {}", e))?;

            let dockerfile_path = absolute_path.join("Dockerfile");
            if dockerfile_path.exists() {
                info!(
                    "Dockerfile detected! Building local image for {}",
                    config.name
                );
                podman.ensure_installed()?;
                podman.ensure_machine_running()?;
                podman.build(&config.name, &absolute_path.to_string_lossy())?;
                config.container_image = Some(config.name.clone());
            }

            // Replace the URL with the absolute path of the directory
            config.url = Some(absolute_path.to_string_lossy().into_owned());

            // Save to global managed directory
            artifacts.create_artifact(&config)?;
            info!("Artifact '{}' loaded and registered", config.name);

            // Start it
            if let Some(ref image) = config.container_image {
                info!("Starting artifact: {}", config.name);
                
                // Force wipe any existing container with this name to ensure network/port sync
                let _ = Command::new("podman").args(["rm", "-f", &config.name]).output();
                
                podman.ensure_installed()?;
                podman.ensure_machine_running()?;
                match podman.run_detached(
                    image,
                    Some(&config.name),
                    config.ports.as_ref(),
                    config.environment.as_ref(),
                ) {
                    Ok(output) => {
                        println!("{} started in background with ID:\n{}", config.name, output);
                        
                        // Register route in Traefik
                        let port = config.gui_window.as_ref().and_then(|g| g.port).unwrap_or(8080);
                        if let Err(e) = TraefikOrchestrator::register_artifact_route(&config.name, port) {
                            error!("Failed to register Traefik route: {}", e);
                        }
                        
                        open_gui_window(&config);
                    }
                    Err(e) => error!("Failed to start {}: {}", config.name, e),
                }
            } else {
                error!(
                    "Artifact '{}' has no container image specified",
                    config.name
                );
            }
        }
        Commands::Viewer { url, width, height, x, y } => {
            uncver_artifacts::gui::run_webview_viewer(&url, width, height, x, y)?;
        }
    }

    Ok(())
}
