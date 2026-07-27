use clap::{Parser, Subcommand};
use std::sync::Arc;

use uncver_artifacts::podman::Podman;
use uncver_artifacts::ArtifactManager;

mod handlers;
mod install;
mod upgrade;

#[derive(Parser)]
#[command(name = "uncver-artifacts", about = "CLI tool for managing uncver artifacts with Podman integration", version = "0.4.0")]
struct Cli {
    #[arg(short, long, global = true, help = "Show detailed build output")]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install,
    List,
    Start { name: String },
    Create {
        #[arg(short, long)] name: String,
        #[arg(short, long)] description: Option<String>,
        #[arg(short, long)] url: Option<String>,
        #[arg(short, long)] local_path: Option<String>,
        #[arg(short, long)] container_image: Option<String>,
    },
    Delete { name: String },
    Watch,
    Run,
    Upgrade { #[arg(long)] force: bool },
    Ps,
    Logs { name: String },
    Refresh,
    Reset,
    Autostart { #[arg(long)] disable: bool },
    #[cfg(feature = "gui")]
    Tray,
    Load { path: String },
    #[cfg(feature = "gui")]
    Viewer {
        url: String,
        #[arg(long)] width: Option<u16>,
        #[arg(long)] height: Option<u16>,
        #[arg(long)] x: Option<i32>,
        #[arg(long)] y: Option<i32>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "warn".into()))
        .init();

    let cli = Cli::parse();
    let podman = Podman::new();
    let artifacts = Arc::new(ArtifactManager::new()?);

    match cli.command {
        Commands::Install => handlers::install::install(&podman, &artifacts, cli.verbose).await,
        Commands::Refresh => handlers::install::refresh(&podman, &artifacts, cli.verbose).await,
        Commands::List => handlers::artifact::list(&artifacts).await,
        Commands::Start { name } => handlers::container::start(&podman, &artifacts, &name).await,
        Commands::Create { name, description, url, local_path, container_image } =>
            handlers::artifact::create(&artifacts, &name, description.as_ref(), url.as_ref(), local_path.as_ref(), container_image.as_ref()),
        Commands::Delete { name } => handlers::artifact::delete(&artifacts, &name),
        Commands::Watch => handlers::artifact::watch(),
        Commands::Run => handlers::container::run(&podman, &artifacts).await,
        Commands::Upgrade { force } => handlers::system::upgrade(force).await,
        Commands::Ps => handlers::container::ps(&podman),
        Commands::Logs { name } => handlers::container::logs(&podman, &name),
        Commands::Reset => handlers::container::reset(),
        Commands::Autostart { disable } => handlers::system::autostart(&podman, disable),
        #[cfg(feature = "gui")]
        Commands::Tray => handlers::system::tray(),
        Commands::Load { path } => handlers::artifact::load(&podman, &artifacts, &path).await,
        #[cfg(feature = "gui")]
        Commands::Viewer { url, width, height, x, y } =>
            handlers::system::viewer(&url, width, height, x, y),
    }
}
