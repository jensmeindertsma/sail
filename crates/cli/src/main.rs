mod command;
mod socket;

use command::{Command, ParseError};
use core::fmt::{self, Formatter};
use owo_colors::OwoColorize;
use socket::{Socket, SocketError};
use std::{env, error::Error, io, process::ExitCode};

const SOCKET_PATH: &str = "/run/sail.socket";

fn main() -> ExitCode {
    let arguments = env::args().skip(1);

    if let Err(error) = run(arguments) {
        print_error(&error);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run(arguments: impl Iterator<Item = String>) -> Result<(), RunError> {
    let command = Command::parse(arguments)?;

    let mut socket = Socket::connect(SOCKET_PATH)?;

    match command {
        Command::Configure { setting, value } => {
            let result = socket.request_configure(setting, value)?;

            match result {
                Ok(_) => println!("configure request succeeded"),
                Err(error) => {
                    return Err(RunError::RequestFailed {
                        reason: format!("failed to configure setting: {error}"),
                    })
                }
            }
        }
        Command::Status => {
            let status = socket.request_status()?;

            println!("Status = {status:#?}");
        }
    }

    Ok(())
}

#[derive(Debug)]
enum RunError {
    Command(ParseError),
    Connect(io::Error),
    RequestFailed { reason: String },
    Socket(SocketError),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Command(parse_error) => write!(f, "failed to parse command: {parse_error}"),
            Self::Connect(io_error) => write!(f, "failed to connect to socket: {io_error}"),
            Self::RequestFailed { reason } => write!(f, "operation failed: {reason}"),
            Self::Socket(socket_error) => {
                write!(f, "failed to send socket request: {socket_error}")
            }
        }
    }
}

impl Error for RunError {}

impl From<ParseError> for RunError {
    fn from(error: ParseError) -> Self {
        Self::Command(error)
    }
}

impl From<io::Error> for RunError {
    fn from(error: io::Error) -> Self {
        Self::Connect(error)
    }
}

impl From<SocketError> for RunError {
    fn from(error: SocketError) -> Self {
        Self::Socket(error)
    }
}

fn print_error(error: impl Error) {
    eprintln!("{}{} {error}", "error".bold().red(), ":".bold())
}
