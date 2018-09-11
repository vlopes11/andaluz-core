all:
	cargo +nightly build
	$(MAKE) -C cli
	$(MAKE) -C wasm

test:
	cargo +nightly test
	$(MAKE) -C cli
	$(MAKE) -C wasm

release:
	cargo +nightly build --release
	$(MAKE) -C cli
	$(MAKE) -C wasm
