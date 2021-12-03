#[cfg(test)]
mod tests {
    use anyhow::Result;
    use http_wasi_wasmtime::OutboundHttp;
    use wasi_cap_std_sync::WasiCtxBuilder;
    use wasi_common::WasiCtx;
    use wasmtime::{Config, Engine, Instance, Linker, Module, Store};

    wit_bindgen_wasmtime::import!("tests/test.wit");

    const HTTP_RUST_TEST: &str =
        "tests/modules/http-rust-hello/target/wasm32-wasi/release/http_rust_hello.wasm";

    #[test]
    fn test_http_allowed() -> Result<()> {
        let allowed = Some(vec!["https://api.brigade.sh".to_string()]);
        test_http(HTTP_RUST_TEST, allowed)
    }

    #[test]
    #[should_panic]
    fn test_http_not_allowed() {
        let allowed = None;
        test_http(HTTP_RUST_TEST, allowed).unwrap();
    }

    fn test_http(wasm: &str, allowed: Option<Vec<String>>) -> Result<()> {
        let wasi = default_wasi();
        let runtime_data = Some(OutboundHttp::new(allowed));
        let test_data = Some(test::TestData::default());
        let ctx = Context {
            wasi,
            runtime_data,
            test_data,
        };

        let imports = |linker: &mut Linker<Context<_>>| {
            http_wasi_wasmtime::add_to_linker(linker, |ctx| -> &mut OutboundHttp {
                ctx.runtime_data.as_mut().unwrap()
            })
        };

        let (store, instance) = instantiate(wasm, ctx, imports)?;
        execute_test(store, instance)
    }

    fn execute_test<T>(mut store: Store<Context<T>>, mut instance: Instance) -> Result<()> {
        let t = test::Test::new(&mut store, &mut instance, |host| {
            host.test_data.as_mut().unwrap()
        })?;

        let _ = t.test(&mut store)?;

        Ok(())
    }

    struct Context<T> {
        wasi: WasiCtx,
        runtime_data: Option<T>,
        test_data: Option<test::TestData>,
    }

    fn instantiate<T>(
        wasm: &str,
        ctx: Context<T>,
        add_imports: impl FnOnce(&mut Linker<Context<T>>) -> Result<()>,
    ) -> Result<(Store<Context<T>>, Instance)> {
        let engine = Engine::new(&default_config()?)?;
        let module = Module::from_file(&engine, wasm)?;
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut Context<T>| &mut cx.wasi)?;
        add_imports(&mut linker)?;

        let mut store = Store::new(&engine, ctx);
        let instance = linker.instantiate(&mut store, &module)?;

        Ok((store, instance))
    }

    fn default_config() -> Result<Config> {
        let mut config = Config::new();
        config.cache_config_load_default()?;
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        Ok(config)
    }

    fn default_wasi() -> WasiCtx {
        WasiCtxBuilder::new().inherit_stdio().build()
    }
}
