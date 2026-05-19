.PHONY: build run check fetch-header clean

build:
	cargo run --manifest-path xtask/Cargo.toml -- build

run:
	cargo run --manifest-path xtask/Cargo.toml -- run

check:
	cargo run --manifest-path xtask/Cargo.toml -- check

fetch-header:
	cargo run --manifest-path xtask/Cargo.toml -- fetch-header

clean:
	cargo clean --manifest-path xtask/Cargo.toml
	rm -rf build
