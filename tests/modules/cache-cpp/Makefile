WASI_CC       ?= /opt/wasi-sdk/bin/clang
WIT_BINDGEN   ?= wit-bindgen
WASMLINK      ?= wasmlink

bindgen:
	$(WIT_BINDGEN) c --import ../../../wit/ephemeral/wasi-cache.wit --out-dir bindings
	$(WIT_BINDGEN) c --export ../../test.wit --out-dir bindings
	
build:
	$(WASI_CC) -I . -I ./bindings -c -o wasi_cache.o bindings/wasi-cache.c
	$(WASI_CC) -I . -I ./bindings -c -o test.o bindings/test.c
	$(WASI_CC) -mexec-model=reactor lib.cpp wasi_cache.o test.o -o ctest.wasm


link: link-fs

link-fs:
	$(WASMLINK) ctest.wasm \
		--interface wasi-cache=../../../wit/ephemeral/wasi-cache.wit \
		--profile wasmtime \
		--module wasi-cache=../../../crates/cache-fs/target/wasm32-wasi/release/wasi_cache_fs.wasm \
		--output ctest-fs-linked.wasm

clean:
	rm *.wasm && rm *.o
