#[cfg(test)]
mod nn_tests {
    use super::runtime::*;
    use anyhow::Result;
    use wasi_nn_tract_wasmtime::{wasi_nn::WasiNnTables, WasiNnTractCtx};
    use wasmtime::*;

    const NN_RUST_TEST: &str = "tests/modules/nn-demo/target/wasm32-wasi/release/nn_demo.wasm";

    #[test]
    fn test_empty_nn() -> Result<()> {
        type WasiNnTable = WasiNnTables<WasiNnTractCtx>;

        let runtime_data = Some((
            WasiNnTractCtx::default(),
            WasiNnTables::<WasiNnTractCtx>::default(),
        ));
        let wasi = default_wasi();
        let test_data = Some(test::TestData::default());
        let ctx = Context {
            wasi,
            runtime_data,
            test_data,
        };

        let add_imports = |linker| {
            wasi_nn_tract_wasmtime::add_to_linker(linker, |ctx: &mut Context<(WasiNnTractCtx, WasiNnTable)>| -> (&mut WasiNnTractCtx, &mut WasiNnTable) {
                let ct = ctx.runtime_data.as_mut().unwrap();
                (&mut ct.0, &mut ct.1)
            })
        };
        let engine = Engine::new(&default_config()?)?;
        let module = Module::from_file(&engine, NN_RUST_TEST)?;
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut Context<_>| &mut cx.wasi)?;
        let mut store = Store::new(&engine, ctx);

        add_imports(&mut linker)?;
        let mut instance = linker.instantiate(&mut store, &module)?;
        let t = test::Test::new(&mut store, &mut instance, |host| {
            host.test_data.as_mut().unwrap()
        })?;
        let _ = t.test(&mut store).expect("Error running the test method");
        Ok(())
    }
}

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
mod cache_tests {
    use super::runtime::*;
    use anyhow::Result;
    use cache_wasi_redis_wasmtime::RedisCache;
    use std::{
        net::{Ipv4Addr, SocketAddrV4, TcpListener},
        process::{Child, Command},
    };
    use wasmtime::Linker;
    use tikv_rust_client_wasmtime::TikvClient;

    const REDIS_SERVER_CLI: &str = "redis-server";
    const TIUP_TIKV_COMMAND: &str = "tiup";
    const CACHE_RUST_TEST: &str =
        "tests/modules/cache-rust/target/wasm32-wasi/release/cache_rust.wasm";
    const CACHE_RUST_LINKED_FS_TEST: &str =
        "tests/modules/cache-rust/target/wasm32-wasi/release/cache_rust_linked_fs.wasm";
    const CACHE_CPP_TEST: &str = "tests/modules/cache-cpp/ctest.wasm";
    const CACHE_CPP_LINKED_FS_TEST: &str = "tests/modules/cache-cpp/ctest-fs-linked.wasm";

    #[tokio::test]
    async fn test_redis_get_set_delete() -> Result<()> {
        init();

        let redis = RedisTestController::new().await?;
        let data = Some(RedisCache::new(&redis.address)?);
        let add_imports = |linker: &mut Linker<Context<_>>| {
            cache_wasi_redis_wasmtime::add_to_linker(linker, |ctx| -> &mut RedisCache {
                ctx.runtime_data.as_mut().unwrap()
            })
        };

        exec(CACHE_CPP_TEST, data.clone(), add_imports)?;
        exec(CACHE_RUST_TEST, data, add_imports)
    }

    #[test]
    fn test_tikv_get_set() -> Result<()> {
        init();

        let controller = TiKVController::new()?;
        let data = Some(TikvClient::new(&controller.pd_endpoint)?);
        let add_imports = |linker: &mut Linker<Context<_>>| {
            tikv_rust_client_wasmtime::add_to_linker(linker, |ctx| -> &mut TikvClient {
                ctx.runtime_data.as_mut().unwrap()
            })
        };

        exec(CACHE_RUST_TEST, data, add_imports);
        Ok(())
    }

    #[test]
    fn test_rust_fs() {
        init();

        let data: Option<u32> = None;
        exec_with_default_imports(CACHE_RUST_LINKED_FS_TEST, data).unwrap();
    }

    #[test]
    fn test_cpp_fs() {
        init();

        let data: Option<u32> = None;
        exec_with_default_imports(CACHE_CPP_LINKED_FS_TEST, data).unwrap();
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

    pub struct TiKVController {
        pub pd_endpoint: String,
        server_handle: Child,
    }

    impl TiKVController {
        pub fn new() -> Result<TiKVController> {
            let server_handle = Command::new(TIUP_TIKV_COMMAND)
                .args(["playground", "--mode", "tikv-slim"])
                .spawn().expect("tiup command failed to start");

            std::thread::sleep(std::time::Duration::from_secs(30));

            Ok(Self {
                pd_endpoint: "127.0.0.1:2379".to_string(),
                server_handle,
            })
        }
    }

    impl Drop for TiKVController {
        fn drop(&mut self) {
            let _ = self.server_handle.kill();
            if let Ok(mut child) = Command::new(TIUP_TIKV_COMMAND).args(["clean", "--all"]).spawn() {
                std::thread::sleep(std::time::Duration::from_secs(10));
                child.kill().expect("command wasn't running");
            }
        }
    }
}

#[cfg(test)]
mod wasi_log_tests {
    use super::runtime::*;
    use anyhow::Result;
    use log_wasmtime::WasiLogger;
    use wasmtime::Linker;

    const RUST_LOG_TEST: &str = "tests/modules/rust-log/target/wasm32-wasi/release/rust_log.wasm";

    #[test]
    fn test_rust_log() -> Result<()> {
        init();

        let data = Some(WasiLogger {});

        let add_imports = |linker: &mut Linker<Context<_>>| {
            log_wasmtime::add_to_linker(linker, |ctx| -> &mut WasiLogger {
                ctx.runtime_data.as_mut().unwrap()
            })
        };

        exec(RUST_LOG_TEST, data, add_imports)
    }
}

mod runtime {
    use anyhow::Result;
    use wasi_cap_std_sync::WasiCtxBuilder;
    use wasi_common::WasiCtx;
    use wasmtime::{Config, Engine, Instance, Linker, Module, Store};
    use wasmtime_wasi::*;

    wit_bindgen_wasmtime::import!("tests/test.wit");

    pub fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    pub fn exec_with_default_imports<T>(wasm: &str, runtime_data: Option<T>) -> Result<()> {
        let ctx = build_ctx(runtime_data);
        let (store, instance) = instantiate_with_default_imports(wasm, ctx)?;
        exec_core(store, instance)
    }

    pub fn instantiate_with_default_imports<T>(
        wasm: &str,
        ctx: Context<T>,
    ) -> Result<(Store<Context<T>>, Instance)> {
        let (_, module, linker, mut store) = emls(wasm, ctx)?;
        let instance = linker.instantiate(&mut store, &module)?;
        Ok((store, instance))
    }

    pub fn exec<T>(
        wasm: &str,
        runtime_data: Option<T>,
        add_imports: impl FnOnce(&mut Linker<Context<T>>) -> Result<()>,
    ) -> Result<()> {
        let ctx = build_ctx(runtime_data);
        let (store, instance) = instantiate(wasm, ctx, add_imports)?;
        exec_core(store, instance)
    }

    pub fn instantiate<T>(
        wasm: &str,
        ctx: Context<T>,
        add_imports: impl FnOnce(&mut Linker<Context<T>>) -> Result<()>,
    ) -> Result<(Store<Context<T>>, Instance)> {
        let (_, module, mut linker, mut store) = emls(wasm, ctx)?;
        add_imports(&mut linker)?;
        let instance = linker.instantiate(&mut store, &module)?;
        Ok((store, instance))
    }

    fn emls<T>(
        wasm: &str,
        ctx: Context<T>,
    ) -> Result<(Engine, Module, Linker<Context<T>>, Store<Context<T>>)> {
        let engine = Engine::new(&default_config()?)?;
        let module = Module::from_file(&engine, wasm)?;
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut Context<T>| &mut cx.wasi)?;
        let store = Store::new(&engine, ctx);
        Ok((engine, module, linker, store))
    }

    fn exec_core<T>(mut store: Store<Context<T>>, instance: Instance) -> Result<()> {
        let t = test::Test::new(&mut store, &instance, |host| {
            host.test_data.as_mut().unwrap()
        })?;
        let result = t.test(&mut store).expect("Error running the test method");
        match result {
            Ok(()) | Err(test::Error::Success) => Ok(()),
            Err(test::Error::Failure) => Err(anyhow::anyhow!("Test returned failure")),
        }
    }

    pub fn default_config() -> Result<Config> {
        let mut config = Config::new();
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        config.wasm_multi_memory(true);
        config.wasm_module_linking(true);
        Ok(config)
    }

    pub fn default_wasi() -> WasiCtx {
        let mut ctx = WasiCtxBuilder::new().inherit_stdio();
        ctx = ctx
            .preopened_dir(
                Dir::open_ambient_dir("./target", ambient_authority()).unwrap(),
                "cache",
            )
            .unwrap();

        ctx.build()
    }

    pub struct Context<T> {
        pub wasi: WasiCtx,
        pub runtime_data: Option<T>,
        pub test_data: Option<test::TestData>,
    }

    fn build_ctx<T>(runtime_data: Option<T>) -> Context<T> {
        let wasi = default_wasi();
        let test_data = Some(test::TestData::default());
        Context {
            wasi,
            runtime_data,
            test_data,
        }
    }
}
