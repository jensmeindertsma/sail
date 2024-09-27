mod command;
mod socket;

use command::Command;
use owo_colors::OwoColorize;
use sail_core::socket::{Failure, SocketRequest, Success};
use socket::Socket;
use std::{
    env,
    io::{self, Write},
    process::ExitCode,
};

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
        Command::SetDashboardHost(host) => {
            let mut socket = Socket::connect(SOCKET_PATH).unwrap();

            let response = socket
                .send_request(SocketRequest::EditDashboardHost { hostname: host })
                .unwrap();

            dbg!(&response);
        }
        Command::SetRegistryHost(host) => {
            let mut socket = Socket::connect(SOCKET_PATH).unwrap();

            let response = socket
                .send_request(SocketRequest::EditRegistryHost { hostname: host })
                .unwrap();

            dbg!(&response);
        }
        Command::List => {
            let mut socket = Socket::connect(SOCKET_PATH).unwrap();

            let response = socket.send_request(SocketRequest::GetApplications).unwrap();

            match response {
                Ok(Success::GetApplications(apps)) => {
                    if apps.is_empty() {
                        println!("No apps!!")
                    }

                    for app in apps {
                        println!("YEAH! app {} with host {}", app.name, app.hostname)
                    }
                }
                _ => panic!("don't fail on me!"),
            }
        }
        Command::Create { name } => {
            println!("let's create app `{name}`");

            let mut socket = Socket::connect(SOCKET_PATH).unwrap();

            if let Err(Failure::NameInUse) = socket
                .send_request(SocketRequest::GetApplication { name: name.clone() })
                .unwrap()
            {
                eprintln!("name `{name}` is already in use ")
            }

            let hostname = prompt("hostname");

            if let Ok(Success::CreatedApplication) = socket
                .send_request(SocketRequest::CreateApplication { name, hostname })
                .unwrap()
            {
                println!("created application :)")
            }
        }
    }

    ExitCode::SUCCESS
}

fn print_error(message: &str, description: Option<&str>) {
    eprintln!("{}{} {}", "error".bold().red(), ":".bold(), message.bold());

    if let Some(description) = description {
        eprintln!("{} {}", "-->".bold().cyan(), description.italic())
    }
}

fn prompt(property: &str) -> String {
    print!("{property}: ");
    io::stdout().flush().unwrap();

    let mut answer = String::new();

    io::stdin().read_line(&mut answer).unwrap();

    answer.trim().to_owned()
}
