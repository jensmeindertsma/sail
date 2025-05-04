mod program;

use std::{
    env,
    process::{ExitCode, Termination},
};

use owo_colors::OwoColorize;

fn main() -> impl Termination {
    if let Err(error) = program::run(env::args().skip(1)) {
        eprintln!(
            "{}{} {}",
            "ERROR".bold().white().on_red(),
            ":".bold(),
            error.bold()
        );

        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
