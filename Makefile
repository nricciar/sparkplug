all: test build

clean:
	cargo clean

test:
	cargo test

build:
	cargo build

release: clean test
	cargo build --release
