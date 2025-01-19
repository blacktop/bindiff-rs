.PHONY: build-deps
build-deps: ## Install build dependencies
	@echo "🔨 Installing Build Dependencies"
	brew install caarlos0/tap/svu
	cargo install --locked cargo-set-version

.PHONY: bump
bump: ## Bump the version of the crate
	@echo "🚀 Bumping Version"
	cargo-set-version set-version $(shell svu patch --strip-prefix)
	git add Cargo.toml
	git commit -m "chore: bump version to $(shell svu patch --strip-prefix)"
	git push
	git tag $(shell svu patch)
	git push --tags

.PHONY: release-dry
release-dry: ## Build the crate for release
	goreleaser build --clean --timeout 60m --snapshot --skip=validate

.PHONY: release
release: bump ## Release the crate to GitHub and crates.io
	goreleaser --clean --timeout 60m --skip=validate

.PHONY: test
test: ## Run the tests
	cargo test 

.PHONY: test-verbose
test-verbose: ## Run the tests with verbose output
	cargo test -- --nocapture

.PHONY: run
run: ## Run the main binary
	cargo run --release tests/kernel.release_vs_kernel.release.BinDiff

.PHONY: example
example: ## Run the example
	cargo run --example=binexport tests/kernel.release.t6020.BinExport

.PHONY: publish-test
publish-test: ## Test publishing the crate to crates.io
	cargo publish --dry-run --allow-dirty

.PHONY: publish
publish: release ## Publish the crate to crates.io
	cargo publish --allow-dirty

.PHONY: clean
clean: ## Clean up artifacts
	cargo clean

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help	
	