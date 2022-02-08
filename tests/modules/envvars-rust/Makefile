WASI_CC       ?= /opt/wasi-sdk/bin/clang
WIT_BINDGEN   ?= wit-bindgen
WASMLINK      ?= wasmlink

link: link-fs

link-fs:
	$(WASMLINK) target/wasm32-wasi/release/envvars_rust.wasm \
		--interface envvars=../../../wit/ephemeral/wasi-envvars.wit \
		--profile wasmtime \
		--module envvars=../../../crates/envvars/target/wasm32-wasi/release/wasi_envvars.wasm \
		--output target/wasm32-wasi/release/envvars_rust_linked.wasm
