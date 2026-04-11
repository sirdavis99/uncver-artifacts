# 🐳 uncver-artifacts

[![Test](https://github.com/sirdavis99/uncver-artifacts/actions/workflows/test.yml/badge.svg)](https://github.com/sirdavis99/uncver-artifacts/actions/workflows/test.yml)
[![Release](https://github.com/sirdavis99/uncver-artifacts/actions/workflows/release.yml/badge.svg)](https://github.com/sirdavis99/uncver-artifacts/actions/workflows/release.yml)
[![Release Please](https://github.com/sirdavis99/uncver-artifacts/actions/workflows/release-please.yml/badge.svg)](https://github.com/sirdavis99/uncver-artifacts/actions/workflows/release-please.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**uncver-artifacts** is a lightning-fast, cross-platform CLI tool built in Rust for managing containerized artifacts powered by **Podman**. It simplifies the lifecycle of local development environments, providing seamless installation, orchestration, and automated updates.

---

## ✨ Features

- 🐳 **Native Podman Integration** — Orchestrate containers without the Docker daemon overhead.
- 📦 **Artifact Management** — Effortlessly list, create, and manage complex artifact configurations.
- 🔄 **Autonomous Upgrades** — Built-in `upgrade` command keeps your CLI and dependencies synchronized.
- 👀 **Smart File Watching** — High-performance watcher reloads your artifacts instantly upon file changes.
- 🏁 **True Cross-Platform** — Native support for **macOS** (Intel & Apple Silicon), **Linux**, and **Windows**.
- 🛠️ **Zero-Config Setup** — Single-command dependency installation with `uncver-artifacts install`.

---

## 🚀 Installation

### 🖥️ Windows (Manual)

1. Download the latest `uncver-artifacts-x86_64-pc-windows-msvc.zip` from [Releases](https://github.com/sirdavis99/uncver-artifacts/releases).
2. Extract the `uncver-artifacts.exe` and add its parent directory to your system `PATH`.

### 🐧 Linux (.deb)

For Debian/Ubuntu users:
```bash
curl -L -O https://github.com/sirdavis99/uncver-artifacts/releases/latest/download/uncver-artifacts-x86_64-unknown-linux-gnu.deb
sudo dpkg -i uncver-artifacts-x86_64-unknown-linux-gnu.deb
```

### 🍏 macOS (.dmg)

Download the `.dmg` file for your architecture ([Intel](https://github.com/sirdavis99/uncver-artifacts/releases/latest/download/uncver-artifacts-x86_64-apple-darwin.dmg) or [Silicon](https://github.com/sirdavis99/uncver-artifacts/releases/latest/download/uncver-artifacts-aarch64-apple-darwin.dmg)), open it, and drag the binary to your binary path.

### 🍎 macOS (Homebrew)

```bash
brew tap sirdavis99/uncver
brew install uncver-artifacts
```

### 🐧 Linux / macOS (Manual)

```bash
# Auto-detect architecture and download
curl -L https://github.com/sirdavis99/uncver-artifacts/releases/latest/download/uncver-artifacts-$(uname -m)-apple-darwin.tar.gz | tar -xz
sudo mv uncver-artifacts /usr/local/bin/
```

---

## 📖 Quick Start

```bash
# 1. Initialize your environment (Installs Podman if missing)
uncver-artifacts install

# 2. Explore available artifacts
uncver-artifacts list

# 3. Create a custom artifact
uncver-artifacts create --name my-app --description "New Rust Service"

# 4. Spin up your default environment
uncver-artifacts run

# 5. Enable developer mode with hot-reload
uncver-artifacts watch
```

---

## 🛠️ CLI Reference

| Command | Usage | Description |
|:---|:---|:---|
| `install` | `uncver-artifacts install` | Sets up Podman and machine dependencies. |
| `list` | `uncver-artifacts list` | Displays all managed artifacts and their status. |
| `start` | `uncver-artifacts start <name>` | Boots a specific artifact container. |
| `create` | `uncver-artifacts create [options]` | Creates a new artifact metadata folder. |
| `delete` | `uncver-artifacts delete <name>` | Removes artifact configuration and data. |
| `watch` | `uncver-artifacts watch` | Watches for file changes to trigger reloads. |
| `run` | `uncver-artifacts run` | Executes the default set of artifacts. |
| `upgrade` | `uncver-artifacts upgrade` | Checks for and installs the latest CLI binary. |

---

## ⚙️ Requirements

- **Podman Desktop / CLI** — Required for container orchestration.
- **macOS Users**: requires a initialized Podman Machine (`podman machine init`).
- **Windows Users**: requires WSL2 backend for Podman.

---

## 🤝 Contributing

We love contributions! Check out our [Contributing Guide](CONTRIBUTING.md) to get started.

## 📄 License

Distributed under the **MIT License**. See `LICENSE` for more information.

## 🔗 Related

- [uncver-kg](https://github.com/sirdavis99/uncver-kg) — Knowledge Graph engine for artifact intelligence.
- [uncver-artifact-lib](https://github.com/uncoverthefuture-org/uncver-artifact-lib) — Core definitions for artifact schemas.

---

Developed with ❤️ by the **uncver** team.
