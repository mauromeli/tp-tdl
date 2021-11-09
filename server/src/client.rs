use std::io::{Read, Write};
use std::net::TcpStream;
use crate::question::Question;

pub struct Client{
    pub(crate) stream: TcpStream
}

pub trait Runnable{
    fn run(client: &mut Client, questions: Vec<Question>);
}
impl Client{
    pub(crate) fn new(stream: TcpStream) -> Client{
        Client{
            stream
        }
    }
}
impl Runnable for Client{
    fn run(client: &mut Client, questions: Vec<Question>) {
        //let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        let bytes_read = client.stream.read(&mut buffer).unwrap();
        //let buffer_as_string = str::from_utf8(&buffer[0..bytes_read]).unwrap();
        let owned_string: String = questions[0].question.to_owned();
        //owned_string.push_str(buffer_as_string);
        //stream.write(owned_string.to_uppercase().as_bytes()).unwrap();
        client.stream.write(owned_string.as_bytes()).unwrap();
    }
}