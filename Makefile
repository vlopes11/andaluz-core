build :
	cargo +nightly build
	$(MAKE) -C web $@

test :
	cargo +nightly test
	$(MAKE) -C web $@

release :
	cargo +nightly build --release
	$(MAKE) -C web $@

clean :
	cargo +nightly clean
	$(MAKE) -C web $@
