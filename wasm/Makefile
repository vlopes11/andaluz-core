build :
	cargo +nightly build --target=wasm32-unknown-unknown --release && \
		wasm-gc ./target/wasm32-unknown-unknown/release/wasm.wasm ./site/wasm.wasm

test:

release: 
	cargo +nightly build --target=wasm32-unknown-unknown --release && \
		wasm-gc ./target/wasm32-unknown-unknown/release/wasm.wasm ./site/wasm.wasm

clean: 
	cargo +nightly clean && \
		rm ./site/wasm.wasm
