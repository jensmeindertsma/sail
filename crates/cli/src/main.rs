use std::{
    io::{self, BufRead, BufReader, Write},
    os::unix::net::UnixStream,
};

fn main() {
    println!("Hello, world!");

    let mut buffer = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut buffer).unwrap();

        let input = buffer.trim();

        let stream = UnixStream::connect("/run/sail.socket").unwrap();

        let mut reader = BufReader::new(stream.try_clone().unwrap()).lines();
        let mut writer = stream;

        writeln!(writer, "{input}").unwrap();
        let response = reader.next().unwrap().unwrap();
        println!("reply = `{response}`");

        buffer.clear();
    }
}
