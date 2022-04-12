WASI_CC       ?= /opt/wasi-sdk/bin/clang
WIT_BINDGEN   ?= wit-bindgen
WASMLINK      ?= wasmlink

link: link-fs

link-fs:
	$(WASMLINK) target/wasm32-wasi/release/cache_rust.wasm \
		--interface wasi-cache=../../../crates/wasi-cache-defs/wit/wasi-cache.wit \
		--profile wasmtime \
		--module wasi-cache=../../../crates/cache-fs/target/wasm32-wasi/release/wasi_cache_fs.wasm \
		--output target/wasm32-wasi/release/cache_rust_linked_fs.wasm
