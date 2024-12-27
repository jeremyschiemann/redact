doc-open:
	cargo doc --no-deps --package redactrs --all-features --open

doc:
	cargo doc --no-deps --package redactrs --all-features

test:
	cargo test --all-features

clippy:
	cargo clippy --all-features

check:
	cargo check --all-features