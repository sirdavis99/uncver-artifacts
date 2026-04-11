# uncver-artifacts

[![Test](https://github.com/sirdavis99/uncver-artifacts/actions/workflows/test.yml/badge.svg)](https://github.com/sirdavis99/uncver-artifacts/actions/workflows/test.yml)
[![Release](https://github.com/sirdavis99/uncver-artifacts/actions/workflows/release.yml/badge.svg)](https://github.com/sirdavis99/uncver-artifacts/actions/workflows/release.yml)
[![Release Please](https://github.com/sirdavis99/uncver-artifacts/actions/workflows/release-please.yml/badge.svg)](https://github.com/sirdavis99/uncver-artifacts/actions/workflows/release-please.yml)

CLI tool for managing uncver artifacts with Podman integration. Built in Rust.

## Features

- 🐳 **Podman Integration** - Manage containers with ease
- 📦 **Artifact Management** - Create, start, delete artifacts
- 🔄 **Self-Upgrading** - Built-in `upgrade` command
- 👀 **File Watching** - Auto-reload on changes
- 🖥️ **Cross-Platform** - macOS (Intel & Apple Silicon), Linux, and Windows

## Installation

### Windows (Manual)

Download the latest `uncver-artifacts-x86_64-pc-windows-msvc.zip` from the [Releases](https://github.com/sirdavis99/uncver-artifacts/releases) page and extract `uncver-artifacts.exe` to a folder in your PATH.

### macOS (Homebrew)

```bash
brew tap sirdavis99/uncver
brew install uncver-artifacts
```

### macOS/Linux (Manual)

```bash
# Download latest release
curl -L -o uncver-artifacts.tar.gz \
  https://github.com/sirdavis99/uncver-artifacts/releases/latest/download/uncver-artifacts-$(uname -m)-apple-darwin.tar.gz

# Extract and install
tar -xzf uncver-artifacts.tar.gz
sudo mv uncver-artifacts /usr/local/bin/
```

### Build from Source

```bash
git clone https://github.com/sirdavis99/uncver-artifacts.git
cd uncver-artifacts
cargo build --release
sudo cp target/release/uncver-artifacts /usr/local/bin/
```

## Quick Start

```bash
# 1. Install dependencies (Podman)
uncver-artifacts install

# 2. List available artifacts
uncver-artifacts list

# 3. Create a new artifact
uncver-artifacts create --name my-artifact --description "My artifact"

# 4. Run default artifacts
uncver-artifacts run

# 5. Watch for changes
uncver-artifacts watch
```

## Commands

| Command | Description |
|---------|-------------|
| `install` | Install and setup Podman dependencies |
| `list` | List all artifacts |
| `start <name>` | Start an artifact by name |
| `create` | Create a new artifact |
| `delete <name>` | Delete an artifact |
| `watch` | Watch artifacts directory for changes |
| `run` | Run all default artifacts |
| `upgrade` | Upgrade to latest version |

## Upgrading

```bash
# Check for and install updates
uncver-artifacts upgrade

# Force reinstall
uncver-artifacts upgrade --force
```

## Requirements

- **Podman** - Container engine ([Installation Guide](https://podman.io/getting-started/installation))
- **macOS**: Podman Machine required (`podman machine init && podman machine start`)

## Documentation

- [Installation Guide](INSTALL.md) - Detailed installation instructions
- [Contributing](CONTRIBUTING.md) - How to contribute

## License

MIT License - see [LICENSE](LICENSE) for details.

## Related Projects

- [uncver-artifact-lib](https://github.com/uncoverthefuture-org/uncver-artifact-lib) - Artifact library and definitions
- [uncver-kg](https://github.com/sirdavis99/uncver-kg) - Knowledge graph for artifact management
