check:
	cargo fmt
	cargo clippy --all-targets --all-features
	cargo test

build:
	cargo build

run:
	cargo run
