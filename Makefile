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
	