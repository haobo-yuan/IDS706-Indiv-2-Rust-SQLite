# Reference: https://github.com/nogibjj/Jeremy_Tan_IDS706_Week8_Individual/blob/main/Makefile

# Format code using rustfmt
format:
	cargo fmt --quiet

# Run clippy for linting
lint:
	cargo clippy --quiet

# Run tests
test:
	cargo test --quiet

# Build and run the project
run:
	cargo run

# Build release version
release:
	cargo build --release

# Install Rust toolchain
install:
	rustup update stable
	rustup default stable 

# Run all formatting, linting, and testing tasks
all: format lint test run
