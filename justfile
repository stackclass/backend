# List all available commands
default:
    just --list

# Build the project
build:
    cargo build --workspace --all-features --all-targets

# Clean the build artifacts
clean:
    cargo clean --verbose

# Linting
clippy:
   cargo clippy --workspace --all-features --all-targets -- -D warnings

# Check formatting
check-fmt:
    cargo +nightly fmt --all -- --check

# Fix formatting
fmt:
    cargo +nightly fmt --all

# Test the project
test:
    cargo test --workspace --all-features --all-targets

# Run all the checks
check:
    just check-fmt
    just clippy
    just test

# Generate OpenAPI documentation
openapi:
    cargo run --bin openapi-generator > openapi.json

# Initialize and update submodules (for first-time setup)
init-submodules:
    git submodule update --init --recursive

# Update git submodules
update-submodules:
    git submodule update --remote --merge

# Run all commend in the local environment
all:
    just clean
    just check
    just build
    just openapi

# Run the main program
run:
    unset https_proxy http_proxy all_proxy && cargo run --bin stackclass-server
