mod socket;

use std::{
    env,
    process::{ExitCode, Termination},
};

use owo_colors::OwoColorize;
use sail_core::socket::SocketRequest;
use socket::{Socket, SocketConnectError, SocketError};

const SOCKET_PATH: &str = "/run/sail.socket";

fn main() -> impl Termination {
    let arguments: Vec<String> = env::args().skip(1).collect();

    println!("Arguments: {arguments:?}");

    let mut socket = match Socket::connect(SOCKET_PATH) {
        Ok(s) => s,
        Err(SocketConnectError(error)) => {
            print_error(&format!("failed to connect to socket: {error:?}"), None);
            return ExitCode::FAILURE;
        }
    };

    println!("Sending greeting ...");

    let response = match socket.send_request(SocketRequest::Greeting) {
        Ok(r) => r,
        Err(socket_error) => {
            print_error(
                "failed to send request",
                Some(&match socket_error {
                    SocketError::FailedDeserialization(de_error) => {
                        format!("failed to deserialize reply: {de_error:?}")
                    }
                    SocketError::FailedSerialization(se_error) => {
                        format!("failed to serialize request: {se_error:?}")
                    }
                    SocketError::NoReply => "received no reply from daemon".to_owned(),
                    SocketError::ReadFailure(io_error) => {
                        format!("failed to read from the socket: {io_error:?}")
                    }
                    SocketError::ReplyMismatch => "incoming reply has ID mismatch".to_owned(),
                    SocketError::WriteFailure(io_error) => {
                        format!("failed to write to the socket: {io_error:?}")
                    }
                }),
            );
            return ExitCode::FAILURE;
        }
    };

    println!("Response to greeting = {response:?}");

    ExitCode::SUCCESS
}

fn print_error(message: &str, description: Option<&str>) {
    eprintln!("{}{} {}", "error".bold().red(), ":".bold(), message.bold());

    if let Some(description) = description {
        eprintln!("{} {}", "-->".bold().cyan(), description.italic())
    }
}
