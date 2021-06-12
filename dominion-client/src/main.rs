use std::io::{self, BufReader, BufRead, Write};
use std::net::TcpStream;
use std::str;

#[tokio::main]
async fn main() {
    let mut socket = TcpStream::connect("localhost:8080").unwrap();

    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        socket.write(input.as_bytes()).unwrap();

        let mut reader = BufReader::new(&socket);

        reader.read_until(b'\n', &mut buffer).unwrap();
        print!("{}", str::from_utf8(&buffer).unwrap());
    }
}
