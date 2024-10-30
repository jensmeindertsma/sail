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
        eprintln!("{}{} {error}", "error".bold().red(), ":".bold());
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run(arguments: impl Iterator<Item = String>) -> Result<(), Failure> {
    let command = Command::parse(arguments)?;

    let mut socket = Socket::connect(SOCKET_PATH).map_err(Failure::SocketConnection)?;

    match command {
        Command::Configure { setting, value } => {
            let result = socket.request_configure(setting, value)?;

            match result {
                Ok(_) => println!("configure request succeeded"),
                Err(error) => {
                    return Err(Failure::RequestFailed {
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
enum Failure {
    CannotParseCommand(ParseError),
    SocketConnection(io::Error),
    RequestFailed { reason: String },
    Socket(SocketError),
}

impl From<ParseError> for Failure {
    fn from(error: ParseError) -> Self {
        Self::CannotParseCommand(error)
    }
}

impl From<SocketError> for Failure {
    fn from(error: SocketError) -> Self {
        Self::Socket(error)
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::CannotParseCommand(parse_error) => {
                write!(f, "failed to parse command: {parse_error}")
            }
            Self::SocketConnection(io_error) => {
                write!(f, "failed to connect to socket: {io_error}")
            }
            Self::RequestFailed { reason } => write!(f, "{reason}"),
            Self::Socket(socket_error) => {
                write!(f, "failed to send socket request: {socket_error}")
            }
        }
    }
}

impl Error for Failure {}
