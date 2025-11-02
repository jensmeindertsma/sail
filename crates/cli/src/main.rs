#![feature(iter_intersperse)]

use std::{
    env,
    io::{BufRead, BufReader, Write},
    os::unix::net::UnixStream,
};

fn main() {
    let input: String = env::args()
        .nth(1)
        .into_iter()
        .intersperse(" ".to_owned())
        .collect();

    let mut stream = UnixStream::connect("/run/sail.socket").unwrap();

    // Send a message (with newline)
    writeln!(stream, "{input}").unwrap();

    // Read reply
    let mut reader = BufReader::new(stream);
    let mut reply = String::new();
    reader.read_line(&mut reply).unwrap();

    println!("Daemon replied: {}", reply.trim());
}
