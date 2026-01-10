.PHONY: build ui release test clean help

# Default target
help:
	@echo "Pillar Build System"
	@echo ""
	@echo "Usage:"
	@echo "  make ui        - Build Web UI static assets"
	@echo "  make build     - Build Rust binary (debug)"
	@echo "  make release   - Full production build (UI + Rust release)"
	@echo "  make test      - Run all tests"
	@echo "  make clean     - Clean build artifacts"

ui:
	@echo "Building Web UI..."
	cd services/ui && npm install && npm run build

build:
	@echo "Building Rust binary (debug)..."
	cargo build

release: ui
	@echo "Building Pillar release binary..."
	cargo build --release
	@echo ""
	@echo "Release build complete: ./target/release/pillar"

test:
	@echo "Running tests..."
	cargo test -- --test-threads=1

clean:
	@echo "Cleaning artifacts..."
	cargo clean
	rm -rf services/ui/dist
	rm -rf services/ui/node_modules
