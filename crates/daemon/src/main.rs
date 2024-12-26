use sail_core::SocketResponse;
use socket::Socket;

mod socket;

#[tokio::main]
async fn main() {
    let socket = Socket::attach();

    loop {
        let mut connection = socket.accept().await;

        tokio::spawn(async move {
            connection.receive().await;
            connection.send(SocketResponse::Welcome).await;
        });
    }
}
