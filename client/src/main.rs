use std::io::{Read, Write};
use std::net::TcpStream;
use std::{io, str};

fn main() {
    let mut stream = TcpStream::connect("localhost:3001").unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    stream.write(buffer.as_bytes()).unwrap();

    let mut recv_buffer = [0; 1024];
    let bytes_received = stream.read(&mut recv_buffer).unwrap();
    println!("Got response from server:{:?}", str::from_utf8(&mut recv_buffer[0..bytes_received-1]).unwrap());
}
