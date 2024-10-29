use tokio::{
    io,
    net::{TcpListener, ToSocketAddrs},
};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn bind(address: impl ToSocketAddrs) -> Result<Self, io::Error> {
        let listener = TcpListener::bind(address).await?;

        Ok(Self { listener })
    }
}

enum ServerError {
    Bind(io::Error),
    SocketFailed {
        error: SocketError,
        remaining_connections: GracefulShutdown,
    },
    SocketStopped {
        remaining_connections: GracefulShutdown,
    },
}

async fn hello_world(_: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
