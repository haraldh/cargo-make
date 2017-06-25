//! # cli
//!
//! Handles the command line arguments and executes the runner.
//!

use clap::{App, Arg, SubCommand};
use descriptor;
use log;
use runner;

/// Handles the command line arguments and executes the runner.
pub fn run_cli() {
    let name = "make";
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    let description = env!("CARGO_PKG_DESCRIPTION");

    let default_toml = "Makefile.toml";

    let mut build_file_arg = "-b, --buildFile=[FILE] 'Build toml file containing the build descriptor (default: ".to_string();
    build_file_arg.push_str(&default_toml);
    build_file_arg.push_str(" if exists)'");

    let matches = App::new("cargo")
        .subcommand(
            SubCommand::with_name(name)
                .version(version)
                .author(author)
                .about(description)
                .arg_from_usage(&build_file_arg)
                .arg_from_usage("-t, --task=[TASK NAME] 'The task name to execute (default: default)'")
                .arg(Arg::from_usage("-l, --loglevel=[LOG LEVEL] 'The log level (default: info)'").possible_values(&["verbose", "info", "error"]))
        )
        .get_matches();

    match matches.subcommand_matches(name) {
        Some(cmd_matches) => {
            let build_file = cmd_matches.value_of("buildFile").unwrap_or(&default_toml);
            let task = cmd_matches.value_of("task").unwrap_or("default");
            let log_level = cmd_matches.value_of("loglevel").unwrap_or("info");

            let logger = log::create(log_level);

            logger.info::<()>("Using Build File: ", &[build_file], None);
            logger.info::<()>("Task: ", &[task], None);

            let config = descriptor::load(&build_file, &logger);

            runner::run(&logger, &config, &task);
        }
        None => panic!("cargo-{} not invoked via cargo command.", name),
    }
}