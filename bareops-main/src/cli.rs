use clap::{ArgAction, ArgMatches, Command};
use std::path::PathBuf;

pub fn parse_cmdline() -> Command {
    Command::new("bareops")
        .arg(
            clap::Arg::new("verbosity")
                .short('v')
                .action(ArgAction::Count)
                .help("set log level: error (v), warn (vv), info (vvv), debug (vvvv), trace (vvvvv). Info is default."),
        )
        .subcommand_required(true)
        .subcommand(
        Command::new("run")
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
                    .required(true)
                    .value_parser(clap::value_parser!(PathBuf))
                    .short('p')
                    .long("path")
                    .help(
                        "Set search path for components aka plugins. Can be added multiple times.",
                    ),
            ),
    )
}

pub fn log_level(matches: &ArgMatches) -> log::LevelFilter {
    match matches.get_count("verbosity") {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Error,
        2 => log::LevelFilter::Warn,
        3 => log::LevelFilter::Info,
        4 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace, // more than 4 'v's
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cmdline_run() {
        let matches = parse_cmdline().try_get_matches_from(vec![
            "bareops", "-vvv", "run", "-f", "file", "-p", "path1", "-p", "path2",
        ]);
        assert!(
            matches.is_ok(),
            "Expected matches, got {:?}",
            matches.err().unwrap()
        );
    }

    #[test]
    fn test_parse_cmdline_run_fail() {
        let matches = parse_cmdline().try_get_matches_from(vec!["bareops", "run", "-f"]);
        assert!(matches.is_err(), "Expected no matches");
    }

    #[test]
    fn test_parse_cmdline_no_command_fail() {
        let matches = parse_cmdline().try_get_matches_from(vec!["bareops", "-v"]);
        assert!(matches.is_err(), "Expected no matches");
    }

    #[test]
    fn test_log_level() {
        let matches = parse_cmdline()
            .try_get_matches_from(vec![
                "bareops", "run", "-f", "file", "-p", "path1", "-p", "path2",
            ])
            .unwrap();
        assert_eq!(log_level(&matches), log::LevelFilter::Info);
        let matches = parse_cmdline()
            .try_get_matches_from(vec![
                "bareops", "-v", "run", "-f", "file", "-p", "path1", "-p", "path2",
            ])
            .unwrap();
        assert_eq!(log_level(&matches), log::LevelFilter::Error);
        let matches = parse_cmdline()
            .try_get_matches_from(vec![
                "bareops", "-vv", "run", "-f", "file", "-p", "path1", "-p", "path2",
            ])
            .unwrap();
        assert_eq!(log_level(&matches), log::LevelFilter::Warn);
        let matches = parse_cmdline()
            .try_get_matches_from(vec![
                "bareops", "-vvv", "run", "-f", "file", "-p", "path1", "-p", "path2",
            ])
            .unwrap();
        assert_eq!(log_level(&matches), log::LevelFilter::Info);
        let matches = parse_cmdline()
            .try_get_matches_from(vec![
                "bareops", "-vvvv", "run", "-f", "file", "-p", "path1", "-p", "path2",
            ])
            .unwrap();
        assert_eq!(log_level(&matches), log::LevelFilter::Debug);
        let matches = parse_cmdline()
            .try_get_matches_from(vec![
                "bareops", "-vvvvv", "run", "-f", "file", "-p", "path1", "-p", "path2",
            ])
            .unwrap();
        assert_eq!(log_level(&matches), log::LevelFilter::Trace);
        let matches = parse_cmdline()
            .try_get_matches_from(vec![
                "bareops", "-vvvvvv", "run", "-f", "file", "-p", "path1", "-p", "path2",
            ])
            .unwrap();
        assert_eq!(log_level(&matches), log::LevelFilter::Trace);
    }
}
