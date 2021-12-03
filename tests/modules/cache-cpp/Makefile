WASI_CC       ?= /opt/wasi-sdk/bin/clang
WIT_BINDGEN   ?= wit-bindgen
WASMLINK      ?= wasmlink

bindgen:
	$(WIT_BINDGEN) c --import ../../../wit/ephemeral/wasi_cache.wit --out-dir bindings
	$(WIT_BINDGEN) c --export ../../test.wit --out-dir bindings
	
build:
	$(WASI_CC) -I . -I ./bindings -c -o wasi_cache.o bindings/wasi_cache.c
	$(WASI_CC) -I . -I ./bindings -c -o test.o bindings/test.c
	$(WASI_CC) -mexec-model=reactor lib.cpp wasi_cache.o test.o -o ctest.wasm


link: link-fs

link-fs:
	$(WASMLINK) ctest.wasm \
		--interface wasi_cache=../../../wit/ephemeral/wasi_cache.wit \
		--profile wasmtime \
		--module wasi_cache=../../../crates/cache-fs/target/wasm32-wasi/release/wasi_cache_fs.wasm \
		--output ctest-fs-linked.wasm

clean:
	rm *.wasm && rm *.o
