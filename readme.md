# WASI Experimental Toolkit

<div align="center">
  <h1>WASI Experimental Toolkit</h1>
  <img src="./docs/images/wasi-toolkit.png" width="300"/>
  <p>The goal of this proposal is to standardize the interfaces for functionality
commonly used from WebAssembly modules, particularly running in non-browser
environments.
  </p>
</div>

Interfaces:

- caching
- logging
- outbound HTTP
- environment variables

The interfaces in this repository are at a very early stages of development, and
are intended to serve as a start for the standardization effort. Improvements
and suggestions are highly encouraged at this point. A secondary goal of this
repository is also to follow the development of the interface types proposal and
[`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen), and be in sync
with the updates from that repository.

### Building and testing

```
$ cargo build
$ cargo test --all -- --nocapture
```

### Prerequisites

- Redis installed so that `redis-server` is available
- [WASI SDK](https://github.com/WebAssembly/wasi-sdk) at
  [v12+](https://github.com/WebAssembly/wasi-sdk/releases/tag/wasi-sdk-14) in
  `/opt/wasi-sdk/` (configurable in
  [`Makefile`](tests/modules/cache-cpp/Makefile))
- [Rust](https://www.rust-lang.org/) at
  [1.56+](https://www.rust-lang.org/tools/install) with the `wasm32-wasi` and `wasm32-unknown-unknown` targets
  configured
