build :
	cargo +nightly build
	$(MAKE) -C wasm $@

test :
	cargo +nightly test
	$(MAKE) -C wasm $@

release :
	cargo +nightly build --release
	$(MAKE) -C wasm $@

clean :
	cargo +nightly clean
	$(MAKE) -C wasm $@
