#!/usr/bin/env bash

set -euxo pipefail

mise trust
mise install

rustup target add wasm32-unknown-unknown

# Install cargo utilities needed for local development, including cargo-watch
# These are also installed in CI via a similar cargo binstall step
cargo binstall -y \
    cargo-nextest \
    cargo-deny \
    cargo-audit \
    cargo-llvm-cov \
    cargo-udeps \
    cargo-watch \
    trunk \
    leptosfmt \
    cargo-generate

pre-commit install --hook-type pre-commit

# Platform-specific setup
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS specific setup
    echo "Setting up macOS environment"
    if command -v brew &> /dev/null; then
        brew install libiconv llvm || true
    fi
fi

(cd apps/ezmempass-app && bun install)

echo "Development environment setup complete!"
