build :
	cargo web build --target=wasm32-unknown-unknown --release && \
		mkdir -p dist && \
		cp ./target/wasm32-unknown-unknown/release/*.js ./dist/ && \
		wasm-gc ./target/wasm32-unknown-unknown/release/*.wasm ./dist/andaluz-wasm.wasm

test:

release: 
	cargo web build --target=wasm32-unknown-unknown --release && \
		mkdir -p dist && \
		cp ./target/wasm32-unknown-unknown/release/*.js ./dist/ && \
		wasm-gc ./target/wasm32-unknown-unknown/release/*.wasm ./dist/andaluz-wasm.wasm

clean: 
	cargo +nightly clean && \
		rm ./dist/*
