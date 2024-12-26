use sail_core::{SocketRequest, SocketResponse};
use std::{env, os::fd::FromRawFd};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines},
    net::{
        UnixListener, UnixStream,
        unix::{OwnedReadHalf, OwnedWriteHalf},
    },
};

pub struct Socket {
    listener: UnixListener,
}

impl Socket {
    pub fn attach() -> Self {
        let var = env::var("LISTEN_FDS").unwrap();

        let fd_count: i32 = var.parse().unwrap();

        if fd_count != 1 {
            panic!("Unexpected value for `LISTEN_FDS`!");
        }

        // SAFETY: this file descriptor comes from systemd
        // For more detail, see https://www.man7.org/linux/man-pages/man3/sd_listen_fds.3.html
        let std_listener = unsafe { std::os::unix::net::UnixListener::from_raw_fd(3) };

        std_listener.set_nonblocking(true).unwrap();

        let listener = UnixListener::from_std(std_listener).unwrap();

        Self { listener }
    }

    pub async fn accept(&self) -> SocketConnection {
        let (stream, _) = self.listener.accept().await.unwrap();

        SocketConnection::new(stream)
    }
}

pub struct SocketConnection {
    reader: Lines<BufReader<OwnedReadHalf>>,
    writer: OwnedWriteHalf,
}

impl SocketConnection {
    pub fn new(stream: UnixStream) -> Self {
        let (reader, writer) = stream.into_split();
        let reader = BufReader::new(reader).lines();

        Self { reader, writer }
    }

    pub async fn receive(&mut self) -> SocketRequest {
        serde_json::from_str(&self.reader.next_line().await.unwrap().unwrap()).unwrap()
    }

    pub async fn send(&mut self, response: SocketResponse) {
        self.writer
            .write_all(serde_json::to_string(&response).unwrap().as_bytes())
            .await
            .unwrap();
    }
}
