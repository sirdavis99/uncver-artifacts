#!/bin/bash
set -e

echo "Installing Podman..."

install_macos() {
    if command -v brew &> /dev/null; then
        echo "Using Homebrew to install Podman..."
        brew install podman
    else
        echo "Homebrew not found. Downloading Podman installer..."
        curl -SL https://github.com/containers/podman/releases/latest/download/podman-installer-macos-amd64.pkg -o /tmp/podman.pkg
        sudo installer -pkg /tmp/podman.pkg -target /
        rm /tmp/podman.pkg
    fi
    
    echo "Initializing Podman machine..."
    podman machine init || true
    podman machine start || true
}

install_linux() {
    if command -v apt-get &> /dev/null; then
        echo "Using apt-get to install Podman..."
        sudo apt-get update
        sudo apt-get install -y podman
    elif command -v dnf &> /dev/null; then
        echo "Using dnf to install Podman..."
        sudo dnf install -y podman
    elif command -v yum &> /dev/null; then
        echo "Using yum to install Podman..."
        sudo yum install -y podman
    else
        echo "Using official install script..."
        curl -SL https://get.podman.io | sh
    fi
}

install_windows() {
    echo "Please install Podman from: https://github.com/containers/podman/releases/latest"
    echo "Or use: winget install RedHat.Podman"
}

case "$(uname -s)" in
    Darwin*)
        install_macos
        ;;
    Linux*)
        install_linux
        ;;
    CYGWIN*|MINGW*|MSYS*)
        install_windows
        ;;
    *)
        echo "Unsupported platform"
        exit 1
        ;;
esac

echo "Podman installation complete!"
podman --version
