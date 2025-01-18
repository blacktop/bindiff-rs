.PHONY: test
test:
	cargo test 

.PHONY: test-verbose
test-verbose:
	cargo test -- --nocapture

.PHONY: run
run:
	cargo run --example bindiff --release

.PHONY: clean
clean:
	cargo clean
	