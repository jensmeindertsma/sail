use sail_core::{SocketRequest, SocketResponse};
use std::{
    io::{BufRead, BufReader, Lines, Write},
    os::unix::net::UnixStream,
    path::Path,
};

pub struct Socket {
    reader: Lines<BufReader<UnixStream>>,
    writer: UnixStream,
}

impl Socket {
    pub fn connect(path: impl AsRef<Path>) -> Self {
        let stream = UnixStream::connect(path).unwrap();

        Self {
            reader: BufReader::new(stream.try_clone().unwrap()).lines(),
            writer: stream,
        }
    }

    pub fn send(&mut self, request: SocketRequest) {
        writeln!(self.writer, "{}", serde_json::to_string(&request).unwrap()).unwrap();
    }

    pub fn receive(&mut self) -> SocketResponse {
        let response = self.reader.next().unwrap().unwrap();

        serde_json::from_str(&response).unwrap()
    }
}
