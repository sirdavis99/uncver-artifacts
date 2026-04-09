# Installation

## macOS

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

Download the latest release for your architecture:

```bash
# For Apple Silicon (M1/M2/M3)
curl -L -o uncver-artifacts.tar.gz \
  https://github.com/sirdavis99/uncver-artifacts/releases/latest/download/uncver-artifacts-aarch64-apple-darwin.tar.gz

# For Intel Macs
curl -L -o uncver-artifacts.tar.gz \
  https://github.com/sirdavis99/uncver-artifacts/releases/latest/download/uncver-artifacts-x86_64-apple-darwin.tar.gz

# Extract and install
tar -xzf uncver-artifacts.tar.gz
sudo mv uncver-artifacts /usr/local/bin/
chmod +x /usr/local/bin/uncver-artifacts
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/sirdavis99/uncver-artifacts.git
cd uncver-artifacts

# Build release binary
cargo build --release

# Install locally
cp target/release/uncver-artifacts /usr/local/bin/
```

## Linux

### Using Cargo

```bash
cargo install --git https://github.com/sirdavis99/uncver-artifacts
```

### Manual Installation

```bash
# Download the latest release
curl -L -o uncver-artifacts.tar.gz \
  https://github.com/sirdavis99/uncver-artifacts/releases/latest/download/uncver-artifacts-x86_64-unknown-linux-gnu.tar.gz

# Extract and install
tar -xzf uncver-artifacts.tar.gz
sudo mv uncver-artifacts /usr/local/bin/
chmod +x /usr/local/bin/uncver-artifacts
```

## Prerequisites

### Podman

uncver-artifacts requires Podman to be installed:

**macOS:**
```bash
brew install podman
podman machine init
podman machine start
```

**Linux:**
```bash
# Ubuntu/Debian
sudo apt-get install podman

# Fedora
sudo dnf install podman

# Or let uncver-artifacts install it
uncver-artifacts install
```

## Quick Start

```bash
# 1. Install dependencies (Podman)
uncver-artifacts install

# 2. List available artifacts
uncver-artifacts list

# 3. Run default artifacts
uncver-artifacts run

# 4. Watch for changes
uncver-artifacts watch
```

## Verifying Installation

```bash
# Check version
uncver-artifacts --version

# View help
uncver-artifacts --help

# Check Podman status
podman version
podman machine list
```

## Troubleshooting

### Podman not found

```bash
# macOS
brew install podman
podman machine init
podman machine start

# Or use the install command
uncver-artifacts install
```

### Permission denied

```bash
# Make sure binary is executable
chmod +x /usr/local/bin/uncver-artifacts

# Or install to local bin
mkdir -p ~/.local/bin
cp uncver-artifacts ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"
```

## Uninstalling

```bash
# Homebrew
brew uninstall uncver-artifacts
brew untap sirdavis99/uncver

# Manual
rm /usr/local/bin/uncver-artifacts
```
