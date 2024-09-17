mod command;
mod socket;

use command::Command;
use owo_colors::OwoColorize;
use sail_core::socket::{SocketRequest, SocketResponse, SuccessResponse};
use socket::Socket;
use std::{env, process::ExitCode};

const SOCKET_PATH: &str = "/run/sail.socket";

fn main() -> ExitCode {
    let command = match Command::try_from_arguments(env::args().skip(1)) {
        Ok(command) => command,
        Err(error) => {
            print_error(
                &error.to_string(),
                Some("run `sail help to view a list of available commands`"),
            );
            return ExitCode::FAILURE;
        }
    };

    match command {
        Command::Help => println!("Help is coming (soon)!"),
        Command::List => {
            let mut socket = Socket::connect(SOCKET_PATH).unwrap();

            let response = socket
                .send_request(SocketRequest::ListApplications)
                .unwrap();

            match response {
                SocketResponse::Success(SuccessResponse::ListApplications(apps)) => {
                    if apps.is_empty() {
                        println!("No apps!!")
                    }

                    for app in apps {
                        println!(
                            "YEAH! app {} with host {} and addr {}",
                            app.name, app.hostname, app.address
                        )
                    }
                }
                _ => panic!("don't fail on me!"),
            }
        }
        Command::Create { name } => {
            println!("let's create app `{name}`");

            let mut socket = Socket::connect(SOCKET_PATH).unwrap();

            let response = socket
                .send_request(SocketRequest::CreateApplication {
                    hostname: format!("{name}.kaas.com"),
                    name,
                    address: "127.0.0.1:3301".parse().unwrap(),
                })
                .unwrap();

            println!("got response {response:?}")
        }
    }

    // let response = match socket.send_request(SocketRequest::ListApplications) {
    //     Ok(r) => r,
    //     Err(error) => {
    //         print_error("failed to send request", Some(&error.to_string()));
    //         return ExitCode::FAILURE;
    //     }
    // };

    // println!("Response to greeting = {response:?}");

    ExitCode::SUCCESS
}

fn print_error(message: &str, description: Option<&str>) {
    eprintln!("{}{} {}", "error".bold().red(), ":".bold(), message.bold());

    if let Some(description) = description {
        eprintln!("{} {}", "-->".bold().cyan(), description.italic())
    }
}
