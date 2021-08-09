use anyhow::{anyhow, Result};
use wasi_common::WasiCtx;
use wasmtime::{AsContextMut, Engine, Linker, Module, Store};

mod wapc_functions;
use wapc_functions::*;

struct WasmStore {
    wasi_ctx: WasiCtx,
}

pub fn main() -> Result<()> {
    let engine = Engine::default();
    let mut linker: Linker<WasmStore> = Linker::new(&engine);

    // setup Wasi
    let wasi_ctx_builder = wasmtime_wasi::WasiCtxBuilder::new().inherit_stdio();
    let wasi_ctx = wasi_ctx_builder.build();

    let mut store = Store::new(&engine, WasmStore { wasi_ctx });

    // register Wasi functions
    wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi_ctx).unwrap();

    // register fake waPC functions
    linker.define(
        "wapc",
        "__guest_request",
        guest_request_func(store.as_context_mut()),
    )?;
    linker.define(
        "wapc",
        "__guest_response",
        guest_response_func(store.as_context_mut()),
    )?;
    linker.define(
        "wapc",
        "__guest_error",
        guest_error_func(store.as_context_mut()),
    )?;
    linker.define(
        "wapc",
        "__host_call",
        host_call_func(store.as_context_mut()),
    )?;
    linker.define(
        "wapc",
        "__host_error",
        host_error_func(store.as_context_mut()),
    )?;
    linker.define(
        "wapc",
        "__host_error_len",
        host_error_len_func(store.as_context_mut()),
    )?;
    linker.define(
        "wapc",
        "__host_response",
        host_response_func(store.as_context_mut()),
    )?;
    linker.define(
        "wapc",
        "__host_response_len",
        host_response_len_func(store.as_context_mut()),
    )?;
    linker.define(
        "wapc",
        "__console_log",
        console_log_func(store.as_context_mut()),
    )?;

    if std::env::args().len() != 2 {
        eprintln!("Wrong number of arguments:");
        eprintln!("  swiftwasm-wapc-runner <policy.wasm>");
        std::process::exit(1);
    }
    let module_file = std::env::args()
        .skip(1)
        .next()
        .ok_or_else(|| anyhow!("Cannot get name of the module to load"))?;

    let module = Module::from_file(&engine, module_file.clone())
        .map_err(|e| anyhow!("Cannot open wasm module {}: {:?}", module_file, e))?;

    let instance = linker
        .instantiate(store.as_context_mut(), &module)
        .map_err(|e| anyhow!("Linker error, cannot instantiate module: {:?}", e))?;
    let start = instance
        .get_typed_func::<(), (), _>(&mut store, "_start")
        .map_err(|e| anyhow!("Cannot find _start function: {:?}", e))?;
    match start.call(&mut store, ()) {
        Ok(()) => Ok(()),
        Err(trap) => {
            if let Some(code) = trap.i32_exit_status() {
                match code {
                    0 => Ok(()),
                    _ => Err(anyhow!(
                        "Unexpected exit code from the wasm module: {:?}",
                        trap
                    )),
                }
            } else {
                Err(anyhow!("Unexpected trap: {:?}", trap))
            }
        }
    }
}
