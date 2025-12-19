#!/bin/bash

# Preda SDK - Quick Start Script

set -e

echo "🚀 Preda SDK - Quick Start"
echo "=========================="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "✅ Rust is installed"
echo ""

# Build the project
echo "📦 Building Preda SDK..."
cargo build --release

echo ""
echo "✅ Build complete!"
echo ""

# Run tests
echo "🧪 Running tests..."
cargo test

echo ""
echo "✅ All tests passed!"
echo ""

# Run examples
echo "📚 Running examples..."
echo ""

echo "1️⃣  Example: Create Market"
cargo run --example create_market --release || true

echo ""
echo "2️⃣  Example: Place Position"
cargo run --example place_position --release || true

echo ""
echo "3️⃣  Example: Query BSI"
cargo run --example query_bsi --release || true

echo ""
echo "=========================="
echo "✨ Preda SDK is ready to use!"
echo ""
echo "Next steps:"
echo "  - Read the README.md for documentation"
echo "  - Check examples/ for usage patterns"
echo "  - Review docs/protocol.md for protocol details"
echo "  - Join our community: Discord, Twitter"
echo ""
echo "Happy building! 🎯"
