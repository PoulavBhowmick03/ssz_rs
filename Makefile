.PHONY: pr build test fmt lint clean

## Default
pr: build test fmt lint

build:
	@set -e; cargo build

test:
	cargo test

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

clean:
	cargo clean