# =============================================================================
# filament-mltl Makefile
# =============================================================================

CARGO ?= cargo

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make fmt              - Format with rustfmt"
	@echo "  make fmt-check        - Verify formatting (CI gate)"
	@echo "  make lint             - Clippy with -D warnings"
	@echo "  make test             - cargo test"
	@echo "  make build            - Release build"
	@echo "  make clean            - cargo clean"
	@echo "  make deny             - cargo deny check licenses and sources"
	@echo "  make docs             - cargo doc, denying missing/broken doc links"
	@echo "  make figures          - Regenerate docs/images with ixd build"
	@echo "  make figures-check    - Fail on a missing or stale figure, writes nothing"
	@echo "  make ci               - All CI gates locally (fmt-check + lint + test + deny + docs)"
	@echo ""
	@echo "GitHub Actions is workflow_dispatch-only. Use 'make ci' while iterating"
	@echo "and dispatch hosted CI only for a finalized revision."

# =============================================================================
# Format / Lint / Test
# =============================================================================

.PHONY: fmt
fmt:
	$(CARGO) fmt --all

.PHONY: fmt-check
fmt-check:
	$(CARGO) fmt --all -- --check

.PHONY: lint
lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

.PHONY: test
test:
	$(CARGO) test --all-targets --all-features

.PHONY: build
build:
	$(CARGO) build --release

.PHONY: clean
clean:
	$(CARGO) clean

# =============================================================================
# Supply chain & safety
# =============================================================================

.PHONY: deny
deny:
	$(CARGO) deny check licenses
	$(CARGO) deny check sources

# =============================================================================
# Documentation
# =============================================================================

# cargo doc is a separate lint pass from clippy: rustdoc-only lints such as
# rustdoc::broken_intra_doc_links only fire here, never under `make lint`.
.PHONY: docs
docs:
	RUSTDOCFLAGS="-D warnings" $(CARGO) doc --no-deps --all-features

# =============================================================================
# Documentation figures
# =============================================================================
#
# docs/images/figures.json declares the figures and is built by the ix-diagrams `ixd` command
# (cargo install --locked --git https://github.com/agent-ix/ix-diagrams ix-diagrams).
# `figures-check` writes nothing and fails on a missing or stale figure.

IXD ?= ixd

.PHONY: figures
figures:
	$(IXD) build docs/images/figures.json --out docs/images

.PHONY: figures-check
figures-check:
	$(IXD) build docs/images/figures.json --out docs/images --check

# =============================================================================
# Composite
# =============================================================================

.PHONY: ci
ci: fmt-check lint test deny docs
