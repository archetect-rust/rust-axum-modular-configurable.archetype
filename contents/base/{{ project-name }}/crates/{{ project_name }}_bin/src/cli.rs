use clap::{Arg, ArgAction, ArgMatches, command, Command, value_parser};
use clap::builder::{BoolishValueParser, EnumValueParser};

use crate::traces::TraceFormat;

pub fn arg_matches() -> ArgMatches {
    command!()
        .name("{{ project-name }}")
        .subcommand(
            Command::new("config")
                .about("Configuration Operations")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(Command::new("defaults").about("Displays the default settings"))
                .subcommand(Command::new("merged").about("Displays the effective settings from all merged sources."))
                .subcommand(
                    Command::new("generate")
                        .about("Generate the effective settings in an adjacent yml file, overwriting existing config."),
                ),
        )
        .arg(
            Arg::new("config-file")
                .help("Specifies additional configuration to merge.")
                .long("config-file")
                .short('c')
                .value_parser(EnumValueParser::<TraceFormat>::new())
                .action(ArgAction::Set)
                .env("CONFIG_FILE")
        )
        .arg(
            Arg::new("host")
                .help("The host the server listens on.")
                .long("host")
                .action(ArgAction::Set)
                .env("SERVER_HOST")
        )
        .arg(
            Arg::new("port")
                .help("Port")
                .short('p')
                .long("port")
                .action(ArgAction::Set)
                .value_parser(value_parser!(i64).range(1024..65535))
                .env("SERVER_PORT")
        )
        .arg(
            Arg::new("tracing-format")
                .help("Specify logging format")
                .long("tracing-format")
                .value_parser(EnumValueParser::<TraceFormat>::new())
                .action(ArgAction::Set)
                .env("TRACING_FORMAT")
        )
        .arg(
            Arg::new("tracing-filter")
                .help("Specify logging and tracing level filters")
                .long("tracing-filter")
                .action(ArgAction::Set)
                .env("TRACING_FILTER")
        )
        .get_matches()
}