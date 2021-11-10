use std::io::{Read, Write};
use std::net::TcpStream;
use std::{io, str};

const HOST: &str = "localhost";
const PORT: &str = "3000";

fn main() {
    let addr = &format!("{}:{}", HOST, PORT);
    let mut stream = TcpStream::connect(addr).unwrap();

    let mut recv_buffer = [0; 1024];
    let bytes_received = stream.read(&mut recv_buffer).unwrap();
    println!("{:?}", str::from_utf8(&mut recv_buffer[0..bytes_received]).unwrap());

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.pop(); // Remove newline
    stream.write(buffer.as_bytes()).unwrap();
}
