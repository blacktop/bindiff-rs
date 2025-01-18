.PHONY: build-deps
build-deps:
	@echo "🔨 Installing Build Dependencies"
	brew install caarlos0/tap/svu
	cargo install --locked cargo-set-version

.PHONY: bump
bump:
	@echo "🚀 Bumping Version"
	git tag $(shell svu patch)
	git push --tags

.PHONY: release-dry
release-dry:
	goreleaser build --clean --timeout 60m --snapshot --skip=validate

.PHONY: release
release: bump
	goreleaser --clean --timeout 60m --skip=validate

.PHONY: test
test:
	cargo test 

.PHONY: test-verbose
test-verbose:
	cargo test -- --nocapture

.PHONY: run
run:
	cargo run --example bindiff --release

.PHONY: examples
examples:
	cargo run --example bindiff

.PHONY: publish-test
publish-test:
	cargo publish --dry-run --allow-dirty

.PHONY: publish
publish:
	cargo publish --allow-dirty

.PHONY: clean
clean:
	cargo clean
	