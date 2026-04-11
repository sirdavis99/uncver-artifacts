use clap::{Parser, Subcommand};
use std::sync::Arc;
use tracing::{error, info};

use uncver_artifacts::{ArtifactConfig, ArtifactManager, Podman};

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
    /// Upgrade uncver-artifacts to the latest version
    Upgrade {
        /// Force upgrade even if already on latest version
        #[arg(short, long)]
        force: bool,
    },
    /// Start the system tray indicator
    Tray,
    /// Load and start an artifact from a local directory
    Load {
        /// Path to the directory containing artifact.json
        path: String,
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
                    match podman.run(image) {
                        Ok(output) => println!("Artifact started:\n{}", output),
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

            let mut path = dirs::data_dir().expect("No data dir found");
            path.push("uncver-artifacts");
            path.push("artifacts");

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
                    match podman.run(image) {
                        Ok(output) => println!("{} started:\n{}", artifact.name, output),
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
        Commands::Load { path } => {
            let target_path = std::path::PathBuf::from(&path);
            let absolute_path = std::fs::canonicalize(&target_path)
                .map_err(|e| anyhow::anyhow!("Invalid path or directory not found: {}", e))?;

            let json_path = absolute_path.join("artifact.json");
            if !json_path.exists() {
                anyhow::bail!("No artifact.json found in {:?}", absolute_path);
            }

            info!("Loading artifact config from {:?}", json_path);
            let content = std::fs::read_to_string(&json_path)?;
            let mut config: ArtifactConfig = serde_json::from_str(&content)
                .map_err(|e| anyhow::anyhow!("Invalid artifact.json format: {}", e))?;

            // Replace the URL with the absolute path of the directory
            config.url = Some(absolute_path.to_string_lossy().into_owned());

            // Save to global managed directory
            artifacts.create_artifact(&config)?;
            info!("Artifact '{}' loaded and registered", config.name);

            // Start it
            if let Some(ref image) = config.container_image {
                info!("Starting artifact: {}", config.name);
                podman.ensure_installed()?;
                podman.ensure_machine_running()?;
                match podman.run(image) {
                    Ok(output) => println!("{} started:\n{}", config.name, output),
                    Err(e) => error!("Failed to start {}: {}", config.name, e),
                }
            } else {
                error!(
                    "Artifact '{}' has no container image specified",
                    config.name
                );
            }
        }
    }

    Ok(())
}
