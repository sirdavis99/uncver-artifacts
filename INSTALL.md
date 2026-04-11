# 📥 Installation Guide

This guide provides detailed instructions for installing **uncver-artifacts** on macOS, Linux, and Windows.

---

## 🖥️ Windows

### Manual Installation (Recommended)

1. Navigate to the [Latest Releases](https://github.com/sirdavis99/uncver-artifacts/releases/latest) page.
2. Download `uncver-artifacts-x86_64-pc-windows-msvc.zip`.
3. Extract the `uncver-artifacts.exe` file to a permanent location (e.g., `C:\tools\uncver-artifacts\`).
4. Add that location to your System Environment Variable `Path`:
   - Search for "Edit the system environment variables" in Windows Search.
   - Click **Environment Variables**.
   - Under **System variables**, find `Path` and click **Edit**.
   - Click **New** and paste the folder path.

### Prerequisites (Windows)
- **WSL 2**: Required for Podman. Run `wsl --install` in PowerShell as administrator if you haven't already.
- **Podman**: You can install it via [Podman Desktop](https://podman-desktop.io/) or by running `uncver-artifacts install` after placing the binary.

---

## 🍎 macOS

### Using Homebrew (Recommended)

```bash
# Add the tap
brew tap sirdavis99/uncver

# Install uncver-artifacts
brew install uncver-artifacts

# Verify installation
uncver-artifacts --version
```

### Manual Installation

```bash
# Detect architecture (ARM/Intel) and download
ARCH=$(uname -m)
OS="apple-darwin"
curl -L -o uncver-artifacts.tar.gz \
  "https://github.com/sirdavis99/uncver-artifacts/releases/latest/download/uncver-artifacts-$ARCH-$OS.tar.gz"

# Extract and install
tar -xzf uncver-artifacts.tar.gz
sudo mv uncver-artifacts /usr/local/bin/
chmod +x /usr/local/bin/uncver-artifacts
```

---

## 🐧 Linux

### Manual Installation

```bash
# Download for Linux x86_64
curl -L -o uncver-artifacts.tar.gz \
  https://github.com/sirdavis99/uncver-artifacts/releases/latest/download/uncver-artifacts-x86_64-unknown-linux-gnu.tar.gz

# Extract and install
tar -xzf uncver-artifacts.tar.gz
sudo mv uncver-artifacts /usr/local/bin/
chmod +x /usr/local/bin/uncver-artifacts
```

### Using Cargo

```bash
cargo install --git https://github.com/sirdavis99/uncver-artifacts
```

---

## 🏗️ Building from Source

If you prefer to build the binary yourself:

```bash
git clone https://github.com/sirdavis99/uncver-artifacts.git
cd uncver-artifacts
cargo build --release

# Install locally
sudo cp target/release/uncver-artifacts /usr/local/bin/
```

---

## 🛠️ Post-Installation Setup

Once installed, run the following command to ensure all container dependencies are met:

```bash
uncver-artifacts install
```

This will:
1. Detect if Podman is installed.
2. Initialize the Podman machine (macOS/Windows).
3. Ensure the container runner is ready.

---

## 🔄 Updating

Updating is handled automatically by the CLI:

```bash
uncver-artifacts upgrade
```

To force a re-installation of the latest version:
```bash
uncver-artifacts upgrade --force
```

---

## ❓ Troubleshooting

### "Command not found"
Ensure the directory where you placed `uncver-artifacts` is in your system's `PATH`.

### Podman machine issues (macOS)
If the container environment is unhealthy, try restarting the machine:
```bash
podman machine stop
podman machine start
```

### Windows WSL2 issues
Ensure your WSL distribution is up to date: `wsl --update`.
