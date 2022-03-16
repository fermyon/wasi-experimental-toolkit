from bindings import Cloudevent
import bindings as b
import wasmtime

from cloudevents.http import CloudEvent, to_binary


def run(cloudevent: CloudEvent) -> None:
    store = wasmtime.Store()
    module = wasmtime.Module.from_file(store.engine, "crates/ce/target/wasm32-wasi/release/ce.wasm")
    linker = wasmtime.Linker(store.engine)
    linker.define_wasi()
    wasi = wasmtime.WasiConfig()
    wasi.inherit_stdout()
    wasi.inherit_stderr()
    store.set_wasi(wasi)

    headers, body = to_binary(cloudevent)

    wasm = b.WasiCe(store, linker, module)
    event = Cloudevent.create(store, wasm)
    event.set_id(store, headers["ce-id"])
    event.set_source(store, headers["ce-source"])
    event.set_type(store, headers["ce-type"])
    event.set_specversion(store, headers["ce-specversion"])
    event.set_time(store, headers["ce-time"])
    event.set_data(store, body)

    res = wasm.ce_handler(store, event)

    assert event.get_id(store) == headers["ce-id"]
    assert event.get_source(store) == headers["ce-source"]
    assert event.get_type(store) == headers["ce-type"]
    assert event.get_specversion(store) == headers["ce-specversion"]
    assert event.get_time(store) == headers["ce-time"]
    assert event.get_data(store) == body

    event.drop(store)
    res.value.drop(store)


if __name__ == "__main__":
    # Create a CloudEvent
    # - The CloudEvent "id" is generated if omitted. "specversion" defaults to "1.0".
    attributes = {
        "type": "com.microsoft.steelthread.wasm",
        "source": "https://example.com/event-producer",
    }
    data = {"message": "Hello World!"}
    event = CloudEvent(attributes, data)
    run(event)
