mod command;

use command::{Command, ParseError};
use owo_colors::OwoColorize;
use std::{
    env,
    process::{ExitCode, Termination},
};

fn main() -> impl Termination {
    let arguments = env::args().skip(1);

    let command = match Command::try_from_arguments(arguments) {
        Ok(command) => command,
        Err(error) => {
            let message = match error {
                ParseError::MissingCommandArgument => "no command was specified".to_owned(),
                ParseError::UnknownCommand(command) => format!("unknown command `{command}`"),
            };

            print_error(&message);
            return ExitCode::FAILURE;
        }
    };

    match command {
        Command::Configure { setting, value } => configure(setting, value),
        Command::Uninstall => uninstall(),
        Command::Update => update(),
    }

    ExitCode::SUCCESS
}

fn print_error(message: &str) {
    eprintln!("{}{} {}", "error".bold().red(), ":".bold(), message)
}
