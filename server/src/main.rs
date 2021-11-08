mod file_reader;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3001").unwrap();
    println!("Listening on port 3001");


    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();
        let buffer_as_string = str::from_utf8(&buffer[0..bytes_read]).unwrap();
        stream.write(buffer_as_string.to_uppercase().as_bytes()).unwrap();
    }
}
