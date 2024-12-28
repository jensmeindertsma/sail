mod socket;

use std::env;

use sail_core::{SocketRequest, SocketResponse};
use socket::Socket;

const SOCKET_PATH: &str = "/run/sail.socket";

fn main() {
    let mut arguments = env::args().skip(1);
    let mut socket = Socket::connect(SOCKET_PATH);

    match arguments.next().expect("a command is required!").as_str() {
        "fetch-greeting" => {
            socket.send(SocketRequest::FetchGreeting);

            match socket.receive() {
                SocketResponse::FetchGreeting(Ok(greeting)) => {
                    println!("greeting = {greeting}")
                }

                other => panic!("Got unexpected response: {other:?}"),
            };
        }
        "set-greeting" => {
            let new_greeting = arguments.next().expect("new greeting is required");

            socket.send(SocketRequest::ModifyGreeting(new_greeting));

            match socket.receive() {
                SocketResponse::ModifyGreeting(Ok(())) => println!("success!"),
                _ => panic!("failed!"),
            }
        }
        other => panic!("unknown command {other}"),
    }
}
