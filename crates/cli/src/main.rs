mod socket;

use owo_colors::OwoColorize;
use socket::Socket;
use std::{env, process::ExitCode};

const SOCKET_PATH: &str = "/run/sail.socket";

fn main() -> ExitCode {
    let _arguments = env::args().skip(1);

    println!("connecting to socket");

    let mut socket = match Socket::connect(SOCKET_PATH) {
        Ok(socket) => socket,
        Err(error) => {
            print_error(&format!("failed to connect to socket: {error}"));
            return ExitCode::FAILURE;
        }
    };

    println!("connected to socket, requesting status");

    let status = match socket.request_status() {
        Ok(status) => status,
        Err(error) => {
            print_error(&format!(
                "encountered error while requesting status: {error}"
            ));
            return ExitCode::FAILURE;
        }
    };

    println!("Server status: {status:#?}");

    ExitCode::SUCCESS
}

fn print_error(message: &str) {
    eprintln!("{}{} {}", "error".bold().red(), ":".bold(), message)
}
