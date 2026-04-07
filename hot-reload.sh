#!/bin/bash

# Hot reload script for uncver-artifacts
# Watches for changes in src/ and rebuilds automatically

echo "🔥 Starting hot reload for uncver-artifacts..."
echo "Press Ctrl+C to stop"
echo ""

# Kill any existing instance
pkill -f "uncver-artifacts" 2>/dev/null

# Function to build and run
run_app() {
    echo "🔄 Changes detected, rebuilding..."
    cargo build --quiet 2>&1 | grep -E "^error" || true
    if [ $? -eq 0 ]; then
        echo "✅ Build successful, restarting..."
        pkill -f "uncver-artifacts" 2>/dev/null
        sleep 0.5
        ./target/debug/uncver-artifacts &
    else
        echo "❌ Build failed, check errors above"
    fi
}

# Initial build and run
echo "🚀 Initial build..."
cargo build --quiet 2>&1 | grep -E "^error" || true
./target/debug/uncver-artifacts &

# Watch for changes using fswatch (macOS) or inotifywait (Linux)
if command -v fswatch &> /dev/null; then
    # macOS
    fswatch -o src/ | while read f; do
        run_app
    done
elif command -v inotifywait &> /dev/null; then
    # Linux
    inotifywait -m -r -e modify src/ | while read path action file; do
        run_app
    done
else
    echo "⚠️  Installing fswatch for hot reload..."
    brew install fswatch 2>/dev/null || apt-get install inotify-tools 2>/dev/null || true
    echo "Please install fswatch (macOS: brew install fswatch) or inotify-tools (Linux)"
    echo "For now, manually run: cargo run"
fi
