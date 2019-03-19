build :
	$(MAKE) -C andaluz-wasm $@
	cp andaluz-wasm/dist/*.js app/scripts/ && \
		cp andaluz-wasm/dist/*.wasm app/assets/ && \
		npm run build

test :
	$(MAKE) -C andaluz-wasm $@

release :
	$(MAKE) -C andaluz-wasm $@
	cp andaluz-wasm/dist/*.js app/scripts/ && \
		cp andaluz-wasm/dist/*.wasm app/assets/ && \
		npm run build

clean :
	$(MAKE) -C andaluz-wasm $@
