# Build commands for the Rust version

.PHONY: all build release test clean run-astar run-pastar help

# Default target
all: release

# Build in debug mode
build:
	@echo "Building in debug mode..."
	cargo build

# Build in release mode (optimized)
release:
	@echo "Building in release mode..."
	cargo build --release
	@echo "Copying binaries to bin/..."
	@if not exist bin mkdir bin
	@copy target\release\msa_astar.exe bin\msa_astar 2>nul || echo Built msa_astar
	@copy target\release\msa_pastar.exe bin\msa_pastar 2>nul || echo Built msa_pastar

# Run tests
test:
	@echo "Running tests..."
	cargo test

# Run tests with output
test-verbose:
	@echo "Running tests with output..."
	cargo test -- --nocapture

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	@if exist bin rmdir /s /q bin

# Run serial A-Star on test file
run-astar:
	@echo "Running serial A-Star..."
	cargo run --release --bin msa_astar -- data/seqs/3/synthetic_easy.fasta

# Run parallel A-Star on test file
run-pastar:
	@echo "Running parallel A-Star..."
	cargo run --release --bin msa_pastar -- data/seqs/4/3pmg_ref1.fasta

# Check code without building
check:
	@echo "Checking code..."
	cargo check

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Run clippy linter
lint:
	@echo "Running clippy..."
	cargo clippy -- -D warnings

# Install binaries to ~/.cargo/bin
install:
	@echo "Installing binaries..."
	cargo install --path .

# Show help
help:
	@echo "Available targets:"
	@echo "  all          - Build release version (default)"
	@echo "  build        - Build debug version"
	@echo "  release      - Build optimized release version"
	@echo "  test         - Run tests"
	@echo "  test-verbose - Run tests with output"
	@echo "  clean        - Remove build artifacts"
	@echo "  run-astar    - Run serial A-Star example"
	@echo "  run-pastar   - Run parallel A-Star example"
	@echo "  check        - Check code without building"
	@echo "  fmt          - Format code"
	@echo "  lint         - Run clippy linter"
	@echo "  install      - Install binaries"
	@echo "  help         - Show this help message"
