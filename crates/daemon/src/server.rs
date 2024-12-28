use tokio::net::TcpListener;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn bind() -> Self {
        let listener = TcpListener::bind(("127.0.0.1", 4250)).await.unwrap();

        Self { listener }
    }
}
