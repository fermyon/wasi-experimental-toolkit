WASI_CC       ?= /opt/wasi-sdk/bin/clang
WIT_BINDGEN   ?= wit-bindgen
WASMLINK      ?= wasmlink
CC_FLAGS      ?= -std=gnu++17 -Wall -fno-exceptions
bindgen:
	$(WIT_BINDGEN) c --import ../../../wit/ephemeral/wasi_cache.wit --out-dir bindings
	$(WIT_BINDGEN) c --export ../../test.wit --out-dir bindings
	
build:
	$(WASI_CC) -I . -I ./bindings -c -o wasi_cache.o bindings/wasi_cache.c
	$(WASI_CC) -I . -I ./bindings -c -o test.o bindings/test.c
	$(WASI_CC) -mexec-model=reactor lib.cpp wasi_cache.o test.o -o ctest.wasm

clean:
	rm *.wasm && rm *.o
