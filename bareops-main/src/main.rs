mod cli;

use bareops_error::BareopsError;
use miette::{IntoDiagnostic, NamedSource};
use std::fs::read_to_string;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> miette::Result<()> {
    env_logger::init();
    let cli = cli::parse_cmdline();

    let matches = cli.get_matches();
    log::set_max_level(cli::log_level(&matches));

    match matches.subcommand() {
        Some(("run", matches)) => {
            let check_mode = matches.get_flag("check");
            let file = matches
                .get_one::<PathBuf>("file")
                .ok_or(BareopsError::InvalidState(
                    // should never happen due to required argument
                    "Missing `file` argument".to_string(),
                ))?;
            let search_paths: Vec<&PathBuf> = matches
                .get_many::<PathBuf>("path")
                .ok_or(BareopsError::InvalidState(
                    // should never happen due to required argument
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
                    bareops::run_tasks(tasks, &search_paths)
                        .await
                        .into_diagnostic()
                }
            }
        }
        _ => {
            // should not happen, if clap parses correctly
            Err(BareopsError::InvalidState("Wrong usage".to_string())).into_diagnostic()
        }
    }
}
