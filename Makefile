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

.PHONY: publish
publish:
	cargo publish --dry-run

.PHONY: clean
clean:
	cargo clean
	