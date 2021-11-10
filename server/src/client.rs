use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use crate::question::Question;

pub struct Client{
    stream: TcpStream
}

pub trait Runnable{
    fn run(client: &mut Client, questions: Vec<Question>);
}

impl Client {
    pub fn new(stream: TcpStream) -> Client{
        Client {
            stream
        }
    }

    pub fn recv(&mut self) -> String {
        let mut buffer = [0; 1024];
        let bytes_read = self.stream.read(&mut buffer).unwrap();
        str::from_utf8(&buffer[0..bytes_read]).unwrap().to_string()
    }

    pub fn send(&mut self, str: &String) {
        self.stream.write_all(str.as_bytes()).unwrap();
    }
}

impl Runnable for Client{
    fn run(client: &mut Client, questions: Vec<Question>) {
        let mut buffer = [0; 1024];
        client.stream.read(&mut buffer).unwrap();
        //let buffer_as_string = str::from_utf8(&buffer[0..bytes_read]).unwrap();
        let owned_string: String = questions[0].question.to_owned();
        //owned_string.push_str(buffer_as_string);
        //stream.write(owned_string.to_uppercase().as_bytes()).unwrap();
        client.stream.write(owned_string.as_bytes()).unwrap();
    }
}