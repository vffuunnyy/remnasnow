# RemnaSnow Makefile

.PHONY: all dev release release-opt clean serve serve-release help check fmt clippy test wasm-opt size

DIST_DIR = dist
PKG_DIR = $(DIST_DIR)/pkg
WASM_FILE = $(PKG_DIR)/remnasnow_bg.wasm
WASM_TARGET = wasm32-unknown-unknown

WASM_OPT_LEVEL = -Oz
WASM_OPT_FLAGS = --enable-bulk-memory --enable-nontrapping-float-to-int --enable-sign-ext --enable-mutable-globals

all: release-opt

dev:
	@echo "🔧 Building DEBUG version (configurable parameters)..."
	@rm -rf $(PKG_DIR)
	cargo build --lib --target $(WASM_TARGET) --features configurable
	wasm-bindgen target/$(WASM_TARGET)/debug/remnasnow.wasm --out-dir $(PKG_DIR) --target web
	@echo "✅ Debug build complete!"
	@echo "   Set methods available: set_particle_count, set_gravity, set_depth, etc."

release:
	@echo "🚀 Building RELEASE version (hardcoded parameters)..."
	@rm -rf $(PKG_DIR)
	cargo build --lib --target $(WASM_TARGET) --release
	wasm-bindgen target/$(WASM_TARGET)/release/remnasnow.wasm --out-dir $(PKG_DIR) --target web
	@echo "✅ Release build complete!"
	@echo "   Parameters are hardcoded for maximum performance."

release-opt: release wasm-opt
	@echo "✅ Optimized release build complete!"

wasm-opt:
	@echo "⚡ Running wasm-opt $(WASM_OPT_LEVEL)..."
	@if command -v wasm-opt >/dev/null 2>&1; then \
		BEFORE=$$(stat -f%z $(WASM_FILE) 2>/dev/null || stat -c%s $(WASM_FILE)); \
		wasm-opt $(WASM_OPT_LEVEL) $(WASM_OPT_FLAGS) $(WASM_FILE) -o $(WASM_FILE); \
		AFTER=$$(stat -f%z $(WASM_FILE) 2>/dev/null || stat -c%s $(WASM_FILE)); \
		echo "   Before: $$BEFORE bytes"; \
		echo "   After:  $$AFTER bytes"; \
		echo "   Saved:  $$((BEFORE - AFTER)) bytes"; \
	else \
		echo "⚠️  wasm-opt not found. Install binaryen:"; \
		echo "   cargo install wasm-opt"; \
		echo "   # or"; \
		echo "   apt install binaryen"; \
		echo "   # or"; \
		echo "   brew install binaryen"; \
	fi

release-profiling:
	@echo "📊 Building RELEASE version with profiling..."
	@rm -rf $(PKG_DIR)
	cargo build --lib --target $(WASM_TARGET) --profile dev
	wasm-bindgen target/$(WASM_TARGET)/debug/remnasnow.wasm --out-dir $(PKG_DIR) --target web --debug
	@echo "✅ Profiling build complete!"

check:
	@echo "🔍 Checking both build modes..."
	cargo check --target $(WASM_TARGET)
	cargo check --target $(WASM_TARGET) --features configurable
	@echo "✅ All checks passed!"

fmt:
	@echo "🎨 Formatting code..."
	cargo fmt
	@echo "✅ Code formatted!"

fmt-check:
	cargo fmt --check

clippy:
	@echo "📎 Running clippy..."
	cargo clippy --target $(WASM_TARGET) -- -D warnings
	cargo clippy --target $(WASM_TARGET) --features configurable -- -D warnings
	@echo "✅ Clippy passed!"

clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf $(PKG_DIR)
	@echo "✅ Clean complete!"

serve: dev
	@echo "🌐 Starting local server on http://localhost:8080"
	@cd $(DIST_DIR) && python3 -m http.server 8080

serve-release: release-opt
	@echo "🌐 Starting local server on http://localhost:8080"
	@cd $(DIST_DIR) && python3 -m http.server 8080

size:
	@echo "📦 WASM file size:"
	@if [ -f $(WASM_FILE) ]; then \
		ls -lh $(WASM_FILE); \
		echo ""; \
		echo "Gzipped size:"; \
		gzip -c $(WASM_FILE) | wc -c | xargs -I {} echo "  {} bytes"; \
	else \
		echo "No WASM file found. Run 'make release' first."; \
	fi

# ============================================================================
# HELP
# ============================================================================

help:
	@echo "RemnaSnow Build System"
	@echo ""
	@echo "Build modes:"
	@echo "  make dev              - Debug build with configurable parameters"
	@echo "  make release          - Release build with hardcoded parameters"
	@echo "  make release-opt      - Release build + wasm-opt (default)"
	@echo "  make release-profiling - Release build with profiling info"
	@echo ""
	@echo "Optimization:"
	@echo "  make wasm-opt         - Run wasm-opt on existing WASM file"
	@echo "                          (requires binaryen: cargo install wasm-opt)"
	@echo ""
	@echo "Checks:"
	@echo "  make check            - Check compilation for both modes"
	@echo "  make fmt              - Format code"
	@echo "  make clippy           - Run linter"
	@echo ""
	@echo "Utilities:"
	@echo "  make clean            - Remove build artifacts"
	@echo "  make serve            - Build dev and start HTTP server"
	@echo "  make serve-release    - Build optimized release and start HTTP server"
	@echo "  make size             - Show WASM file size (raw and gzipped)"
	@echo ""
	@echo "Usage:"
	@echo "  Debug mode allows runtime configuration via JS API:"
	@echo "    snowfall.set_gravity(8.0)"
	@echo "    snowfall.set_particle_count(200000)"
	@echo "    etc."
	@echo ""
	@echo "  Release mode has all parameters baked in for max performance."
