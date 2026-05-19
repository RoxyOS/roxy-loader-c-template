.PHONY: build run check fetch-header clean

build:
	cargo run -p xtask -- build

run:
	cargo run -p xtask -- run

check:
	cargo run -p xtask -- check

fetch-header:
	cargo run -p xtask -- fetch-header

clean:
	cargo clean -p xtask
	rm -rf build
