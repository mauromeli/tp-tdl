use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use crate::model::question::Question;
use crate::packages::Package;

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

    pub fn recv(&mut self) -> Package {
        let mut buffer = [0; 1024];
        let bytes_read = self.stream.read(&mut buffer).unwrap();
        println!("{}", str::from_utf8(&buffer[0..bytes_read]).unwrap().to_string());
        decode_package(&buffer).unwrap()
    }

    pub fn send(&mut self, str: String) {
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

fn decode_package(bytes: &[u8]) -> Result<Package, String> {
    match bytes[0] as char {
        'C' => {
            let player_name = str::from_utf8(&bytes[1..]).unwrap().to_string();
            Ok(Package::Connect{ player_name })
        },
        'S' => {
            let player_id = str::from_utf8(&bytes[1..]).unwrap().to_string();
            Ok(Package::StartGame{ player_id })
        },
        'R' => {
            let player_id = str::from_utf8(&bytes[1..2]).unwrap().to_string();
            let response = str::from_utf8(&bytes[2..]).unwrap().to_string();
            Ok(Package::Response{ player_id, response })
        },
        'H' => { //Check status, si se saca el StartGame le cambiamos la letra a S
            let player_id = std::str::from_utf8(&bytes[1..]).unwrap().to_string();
            Ok(Package::CheckStatus{ player_id })
        }
        _ => { Err("Error parseando el paquete enviado".to_string()) }
    }
}
