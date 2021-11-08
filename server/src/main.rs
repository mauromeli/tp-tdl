mod file_reader;
mod question;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3001").unwrap();
    println!("Listening on port 3001");
    let questions: Vec<question::Question> = file_reader::reader(); //we should create game's class
    println!("{}", questions[0].question);
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();
        //let buffer_as_string = str::from_utf8(&buffer[0..bytes_read]).unwrap();
        let owned_string: String = questions[0].question.to_owned();
        //owned_string.push_str(buffer_as_string);
        //stream.write(owned_string.to_uppercase().as_bytes()).unwrap();
        stream.write(owned_string.as_bytes()).unwrap();
    }
}
