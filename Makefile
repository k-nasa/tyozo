build:
	cargo build
run:
	cargo run
test:
	cargo test
server:
	cargo run --bin tyozo-server
cli:
	cargo run --bin tyozo-cli
fix:
	cargo fix --allow-dirty --allow-staged -Z unstable-options --clippy
