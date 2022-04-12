WASI_CC       ?= /opt/wasi-sdk/bin/clang
WIT_BINDGEN   ?= wit-bindgen
WASMLINK      ?= wasmlink

link: link-fs

link-fs:
	$(WASMLINK) target/wasm32-wasi/release/cloudevent_demo.wasm \
		--interface wasi-ce=../../../crates/wasi-ce-defs/wit/wasi-ce.wit \
		--profile wasmtime \
		--module wasi-ce=../../../crates/ce/target/wasm32-wasi/release/ce.wasm \
		--output target/wasm32-wasi/release/ce_linked.wasm
