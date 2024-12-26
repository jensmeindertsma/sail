mod socket;

use sail_core::SocketRequest;
use socket::Socket;

const SOCKET_PATH: &str = "/run/sail.socket";

fn main() {
    let mut socket = Socket::connect(SOCKET_PATH);

    socket.send(SocketRequest::Greeting);

    let response = socket.receive();

    println!("response = {response:?}");
}
