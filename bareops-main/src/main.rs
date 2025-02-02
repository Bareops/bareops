mod cli;
mod error;
mod wasm_runtime;

use crate::error::BareopsError;
use crate::wasm_runtime::WasmRuntime;
use bareops_lang::Task;
use miette::{IntoDiagnostic, NamedSource};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use wasmtime::Config;

#[tokio::main]
async fn main() -> miette::Result<()> {
    env_logger::init();

    let cli = cli::parse_cmdline();

    let matches = cli.get_matches();
    match matches.subcommand() {
        Some(("run", matches)) => {
            let check_mode = matches.get_flag("check");
            let file = matches
                .get_one::<PathBuf>("file")
                .ok_or(BareopsError::TaskbookParse(
                    "Missing `file` argument".to_string(),
                ))?;
            let search_paths: Vec<&PathBuf> = matches
                .get_many::<PathBuf>("path")
                .ok_or(BareopsError::TaskbookParse(
                    "Missing `path` argument".to_string(),
                ))?
                .collect::<Vec<&PathBuf>>();

            // TODO: use tokio, extract to a file handling component
            let source = read_to_string(file).into_diagnostic()?;
            let source = NamedSource::new("demo", source);
            match bareops_lang::parse(&source) {
                Err(e) => Err(e),
                Ok(tasks) => {
                    if check_mode {
                        return Ok(());
                    }
                    run_tasks(tasks, &search_paths).await.into_diagnostic()
                }
            }
        }
        _ => {
            // should not happen, if clap parses correctly
            Err(BareopsError::TaskbookParse("Wrong usage".to_string())).into_diagnostic()
        }
    }
}

async fn run_tasks(
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
        runtime.run_component(task.plugin().name().as_str()).await?
    }
    Ok(())
}
