from dataclasses import dataclass
from tkinter import W
from bindings import Cloudevent
from typing import Tuple, List
import bindings as b
import sys
import wasmtime

def run() -> None:
    store = wasmtime.Store()
    module = wasmtime.Module.from_file(store.engine, "crates/ce/target/wasm32-wasi/release/ce.wasm")
    linker = wasmtime.Linker(store.engine)
    linker.define_wasi()
    wasi = wasmtime.WasiConfig()
    wasi.inherit_stdout()
    wasi.inherit_stderr()
    store.set_wasi(wasi)

    wasm = b.WasiCe(store, linker, module)
    event = Cloudevent.create(store, wasm)
    event.set_id(store, "bbb")
    assert(event.get_id(store) == "bbb")
    event.drop(store)

if __name__ == "__main__":
    run()