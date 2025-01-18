.PHONY: test
test:
	cargo test

.PHONY: run
run:
	cargo run --example bindiff --release

.PHONY: clean
clean:
	cargo clean
	