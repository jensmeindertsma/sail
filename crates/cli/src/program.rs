pub mod socket;

use sail_core::socket::{SocketRequest, SocketResponse};
use socket::{Socket, SocketError};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

const SOCKET_PATH: &str = "/run/sail.socket";

pub fn run(mut arguments: impl Iterator<Item = String>) -> Result<(), ProgramError> {
    let request = match arguments
        .next()
        .ok_or(ProgramError::MissingArgument("command".to_owned()))?
        .as_str()
    {
        "help" => {
            println!("Available commands:");
            println!("- greet <message> // send a greeting to the server");
            println!("- set-greeting <message> // set the greeting the server will reply with");
            return Ok(());
        }
        "greet" => {
            let message = arguments
                .next()
                .ok_or(ProgramError::MissingArgument("message".to_owned()))?;

            SocketRequest::Greet { message }
        }
        "set-greeting" => {
            let message = arguments
                .next()
                .ok_or(ProgramError::MissingArgument("message".to_owned()))?;

            SocketRequest::SetGreeting { message }
        }
        other => return Err(ProgramError::UnknownArgument(other.to_owned())),
    };

    let mut socket = Socket::connect(SOCKET_PATH).map_err(ProgramError::Socket)?;
    let response = socket.send(request).map_err(ProgramError::Socket)?;

    match response {
        SocketResponse::Success => {
            println!("The request completed successfully!")
        }
        SocketResponse::Greeting { message } => {
            println!("Server responded with a greeting {message}");
        }
    };

    Ok(())
}

#[derive(Debug)]
pub enum ProgramError {
    MissingArgument(String),
    Socket(SocketError),
    UnknownArgument(String),
}

impl Display for ProgramError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingArgument(name) => write!(f, "missing argument `{name}`"),
            Self::Socket(socket_error) => write!(f, "socket error: {socket_error}"),
            Self::UnknownArgument(name) => write!(f, "unknown argument `{name}`"),
        }
    }
}

impl Error for ProgramError {}
