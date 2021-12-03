#[cfg(test)]
mod http_tests {
    use super::runtime::*;
    use anyhow::Result;
    use wasi_outbound_http_wasmtime::OutboundHttp;
    use wasmtime::Linker;

    const HTTP_RUST_TEST: &str =
        "tests/modules/http-rust-hello/target/wasm32-wasi/release/http_rust_hello.wasm";

    #[test]
    fn test_http_allowed() -> Result<()> {
        let data = Some(OutboundHttp::new(Some(vec![
            "https://api.brigade.sh".to_string()
        ])));

        let add_imports = |linker: &mut Linker<Context<_>>| {
            wasi_outbound_http_wasmtime::add_to_linker(linker, |ctx| -> &mut OutboundHttp {
                ctx.runtime_data.as_mut().unwrap()
            })
        };

        exec(HTTP_RUST_TEST, data, add_imports)
    }

    #[test]
    #[should_panic]
    fn test_http_not_allowed() {
        let data = Some(OutboundHttp::new(None));

        let add_imports = |linker: &mut Linker<Context<_>>| {
            wasi_outbound_http_wasmtime::add_to_linker(linker, |ctx| -> &mut OutboundHttp {
                ctx.runtime_data.as_mut().unwrap()
            })
        };

        exec(HTTP_RUST_TEST, data, add_imports).unwrap();
    }
}

#[cfg(test)]
mod redis_tests {
    use super::runtime::*;
    use anyhow::Result;
    use cache_wasi_redis_wasmtime::RedisCache;
    use std::{
        net::{Ipv4Addr, SocketAddrV4, TcpListener},
        process::{Child, Command},
    };
    use wasmtime::Linker;

    const REDIS_SERVER_CLI: &str = "redis-server";
    const CACHE_RUST_TEST: &str =
        "tests/modules/cache-rust/target/wasm32-wasi/release/cache_rust.wasm";
    // const CACHE_CPP_TEST: &str = "tests/modules/cache-cpp/ctest.wasm";

    #[tokio::test]
    async fn test_redis_get_set_delete() -> Result<()> {
        let redis = RedisTestController::new().await?;
        let data = Some(RedisCache::new(&redis.address)?);

        let add_imports = |linker: &mut Linker<Context<_>>| {
            cache_wasi_redis_wasmtime::add_to_linker(linker, |ctx| -> &mut RedisCache {
                ctx.runtime_data.as_mut().unwrap()
            })
        };

        // exec(CACHE_CPP_TEST, data, add_imports)?;
        exec(CACHE_RUST_TEST, data.clone(), add_imports)
    }

    pub struct RedisTestController {
        pub address: String,
        server_handle: Child,
    }

    impl RedisTestController {
        pub async fn new() -> Result<RedisTestController> {
            let port = get_random_port();
            let address = format!("redis://localhost:{}", port);

            let server_handle = Command::new(REDIS_SERVER_CLI)
                .args(&["--port", port.to_string().as_str()])
                .spawn()?;

            tokio::time::sleep(std::time::Duration::from_secs(5)).await;

            Ok(Self {
                address,
                server_handle,
            })
        }
    }

    impl Drop for RedisTestController {
        fn drop(&mut self) {
            let _ = self.server_handle.kill();
        }
    }

    fn get_random_port() -> u16 {
        TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
            .expect("Unable to bind to check for port")
            .local_addr()
            .unwrap()
            .port()
    }
}

mod runtime {
    use anyhow::Result;
    use wasi_cap_std_sync::WasiCtxBuilder;
    use wasi_common::WasiCtx;
    use wasmtime::{Config, Engine, Instance, Linker, Module, Store};

    wit_bindgen_wasmtime::import!("tests/test.wit");

    pub fn exec<T>(
        wasm: &str,
        runtime_data: Option<T>,
        add_imports: impl FnOnce(&mut Linker<Context<T>>) -> Result<()>,
    ) -> Result<()> {
        let wasi = default_wasi();
        let test_data = Some(test::TestData::default());
        let ctx = Context {
            wasi,
            runtime_data,
            test_data,
        };

        let (mut store, mut instance) = instantiate(wasm, ctx, add_imports)?;
        let t = test::Test::new(&mut store, &mut instance, |host| {
            host.test_data.as_mut().unwrap()
        })?;

        let _ = t.test(&mut store)?;

        Ok(())
    }

    pub fn instantiate<T>(
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

    pub fn default_config() -> Result<Config> {
        let mut config = Config::new();
        config.cache_config_load_default()?;
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        Ok(config)
    }

    pub fn default_wasi() -> WasiCtx {
        WasiCtxBuilder::new().inherit_stdio().build()
    }

    pub struct Context<T> {
        pub wasi: WasiCtx,
        pub runtime_data: Option<T>,
        pub test_data: Option<test::TestData>,
    }
}
