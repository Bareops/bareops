use bareops_lang::Task;
use error::BareopsError;
use std::path::Path;
use wasm_runtime::WasmRuntime;
use wasmtime::Config;

pub mod error;
mod wasm_runtime;

pub async fn run_tasks(
    tasks: Vec<Task>,
    search_paths: &[impl AsRef<Path>],
) -> Result<(), BareopsError> {
    let mut config = Config::new();
    config.async_support(true);
    config.wasm_component_model(true);
    config.debug_info(true);

    let mut runtime = WasmRuntime::new(config)?;

    runtime.set_search_paths(search_paths);

    for task in tasks {
        runtime
            .run_component(task.plugin().name().as_str(), task.plugin().options())
            .await?
    }
    Ok(())
}
