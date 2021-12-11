use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::str;
use crate::packages::Package;

pub struct Client{
    stream: TcpStream,
    pub addr: SocketAddr
}

impl Client {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Client{
        Client {
            stream,
            addr
        }
    }

    pub fn recv(&mut self) -> Result<Package, &'static str> {
        let mut buffer = [0; 1024];
        //1) 
        let bytes_read = self.stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            Err("Connection closed")
        } else {
            Ok(decode_package(&buffer[0..bytes_read]).unwrap())
        }
    }

    pub fn send(&mut self, str: String) {
        let bytes_to_send = str.as_bytes();
        //2)
        self.stream.write_all(bytes_to_send).unwrap();
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
        'W' => { //Check status
            let player_id = std::str::from_utf8(&bytes[1..]).unwrap().to_string();
            Ok(Package::CheckStatus{ player_id })
        }
        _ => { Err("Error parseando el paquete enviado".to_string()) }
    }
}
