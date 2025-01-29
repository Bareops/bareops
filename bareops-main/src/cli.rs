use clap::{ArgAction, Command};
use std::path::PathBuf;

pub fn parse_cmdline() -> Command {
    clap::Command::new("bareops")
        .subcommand_required(true)
        .subcommand(
            clap::Command::new("run")
                .arg(
                    clap::Arg::new("file")
                        .num_args(1)
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf))
                        .short('f')
                        .long("file")
                        .help("file to run"),
                )
                .arg(
                    clap::Arg::new("check")
                        .num_args(0)
                        .required(false)
                        .long("check")
                        .help("check only (dry-run)"),
                )
                .arg(
                    clap::Arg::new("path")
                        .num_args(1)
                        .action(ArgAction::Append)
                        .required(false)
                        .value_parser(clap::value_parser!(PathBuf))
                        .short('p')
                        .long("path")
                        .help("Set search path for components aka plugins. Can be added multiple times."),
                ),
        )
}
