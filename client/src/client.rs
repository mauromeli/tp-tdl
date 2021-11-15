use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

pub struct Client {}

impl Client {
    pub fn new() -> Client {
        Client {}
    }

    pub fn run(&mut self, host: &str, port: &str) {
        let addr = &format!("{}:{}", host, port);
        let mut stream = TcpStream::connect(addr).unwrap();

        let mut recv_buffer = [0; 1024];
        let mut bytes_received = stream.read(&mut recv_buffer).unwrap();
        println!("{:?}", from_utf8(&mut recv_buffer[0..bytes_received]).unwrap());

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop(); // Remove newline
        stream.write(buffer.as_bytes()).unwrap();

        bytes_received = stream.read(&mut recv_buffer).unwrap();
        println!("{:?}", from_utf8(&mut recv_buffer[0..bytes_received]).unwrap());
    }
}
