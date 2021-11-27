use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::io::stdin;
use crate::packages::Package;
use crate::decode_packages::decode_package;

pub struct Client {}

/*
Cliente:
	- Una vez conectado loopear ping (¿Checkstatus?)
	- Matchear
		- Pregunta -> (generar respuesta) ->Respuesta
		- Fin Partida -> Tabla puntajes
		- WAIT
 */

impl Client {
    pub fn new() -> Client {
        Client {}
    }

    pub fn run(&mut self, host: &str, port: &str) {

        let player_name = self.name_consultor();

        let addr = &format!("{}:{}", host, port);
        let mut stream = TcpStream::connect(addr).unwrap();

        let bytes = [
                "C".to_string().as_bytes(),
                player_name.as_bytes(),
            ].concat();

        stream.write(&bytes);

        let mut recv_buffer = [0; 1024];
        let bytes_received = stream.read(&mut recv_buffer).unwrap();
        //let response = from_utf8(&mut recv_buffer[0..bytes_received]).unwrap();

        let ack_package = decode_package(&mut recv_buffer[0..bytes_received]).unwrap();

        match ack_package {
            Package::ACKConnect { player_id } => {
                println!("Esperando Preguntas.... \n");
                let bytes = [
                        "S".to_string().as_bytes(),
                        player_id.as_bytes(),
                    ].concat();
                stream.write(&bytes);
            },
            _ => {
                println!("couldn't connect");
                return
            }
        }

        let mut recv_buffer = [0; 1024];
        while let Ok(bytes_received) = stream.read(&mut recv_buffer) {
            //let mut bytes_received = stream.read(&mut recv_buffer).unwrap();
            let package = decode_package(&mut recv_buffer[0..bytes_received]);

            if let Ok(Package::Question{ question, options }) = package {
                println!("Pregunta: {}", question);
                println!("Opcion A: {}", options[0]);
                println!("Opcion B: {}", options[1]);
                println!("Opcion C: {}", options[2]);
            }

            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            buffer.pop(); // Remove newline

            let bytes = [
                    "R".to_string().as_bytes(),
                    "1".as_bytes(),
                    buffer.as_bytes(),
                ].concat();
            stream.write(&bytes);

            //println!("{:?}", from_utf8(&mut recv_buffer[0..bytes_received]).unwrap());


            //stream.write(buffer.as_bytes()).unwrap();

            //bytes_received = stream.read(&mut recv_buffer).unwrap();
            //println!("{:?}", from_utf8(&mut recv_buffer[0..bytes_received]).unwrap());
        }
    }

    fn name_consultor(&self) -> String {
        println!("Bienvenido a Kaho-rust!, ¿Cuál es tu nombre?");

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap(); // <- API requires buffer param as of Rust 1.0; returns `Result` of bytes read
        let res = match buffer.trim_end() {
            "" => {
                let name = self.name_generator();
                println!("Oh!, veo que queres mantenerte anonimo. Te llamaremos {}", name);
                name
            }
            name => name.to_string(),
        };

        println!("Hola {}. Mucha suerte :) !!", res);
        res
    }

    fn name_generator(&self) -> String {
        "BOT".to_string()
    }
}
