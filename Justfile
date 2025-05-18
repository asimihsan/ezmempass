set shell := ["bash", "-c"]
set dotenv-load := true

# Default action is to list all available tasks
@default:
    @just --list

# Setup development environment
setup:
    @./tools/dev-bootstrap.sh

# ---- Development Tasks ----

# Watch for changes and run checks
watch:
    cargo watch -x check -x test

# Check the entire workspace
check:
    cargo check --workspace --all-targets

serve-app: clippy
    cd apps/ezmempass-app && trunk serve --open

# ---- Static Analysis ----

# Format code
fmt:
    cargo fmt --all
    leptosfmt --quiet --max-width 100 ./apps/ezmempass-app/**/*.rs

# Check formatting
fmt-check:
    cargo fmt --all --check
    leptosfmt --quiet --max-width 100 --check ./apps/ezmempass-app/**/*.rs

# Run clippy on all workspace crates
clippy:
    cargo clippy --workspace --all-targets -- -D warnings

# Audit dependencies for vulnerabilities
audit:
    cargo audit
    cargo deny check

# Find unused dependencies
udeps:
    cargo +nightly udeps --workspace

# Run all static analysis checks
lint: fmt-check clippy audit

# ---- Testing ----

# Run tests
test:
    cargo nextest run --workspace

# Run tests with coverage
cov:
    cargo llvm-cov nextest --workspace --lcov --output-path lcov.info
    cargo llvm-cov report

# ---- Building ----

# Build all workspace crates
build:
    cargo build --workspace

# Build for release
build-release:
    cargo build --workspace --release

# Build WASM targets
build-wasm:
    cargo build --target wasm32-wasip1
    cargo build --target wasm32-unknown-unknown

# Clean build artifacts
clean:
    cargo clean

# ---- CI Tasks ----

# Run CI checks
ci: fmt-check clippy test
