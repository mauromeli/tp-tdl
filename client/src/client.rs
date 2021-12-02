use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::io::stdin;
use std::{thread, time};
use crate::packages::Package;
use crate::decode_packages::decode_package;

pub struct Client {}

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
        let mut bytes_received = stream.read(&mut recv_buffer).unwrap();
        //let response = from_utf8(&mut recv_buffer[0..bytes_received]).unwrap();

        let ack_package = decode_package(&mut recv_buffer[0..bytes_received]).unwrap();
        let mut player: String;

        match ack_package {
            Package::ACKConnect { player_id } => {
                println!("Esperando Preguntas... Soy {} \n", player_id);
                player = player_id;
            }
            _ => {
                println!("couldn't connect");
                return;
            }
        }

        loop {
            let bytes = [
                "W".to_string().as_bytes(), //Check Status
                player.clone().as_bytes(),
            ].concat();

            stream.write(&bytes);

            let mut recv_buffer = [0; 1024];
            bytes_received = stream.read(&mut recv_buffer).unwrap();

            let package = decode_package(&mut recv_buffer[0..bytes_received]).unwrap();

            match package {
                Package::Question { question, options } => {
                    println!("Pregunta: {}", question);
                    println!("Opcion A: {}", options[0]);
                    println!("Opcion B: {}", options[1]);
                    println!("Opcion C: {}", options[2]);
                    println!("Opcion D: {}", options[3]);

                    let mut buffer = String::new();
                    io::stdin().read_line(&mut buffer).unwrap();
                    buffer.pop(); // Remove newline
                    println!();

                    let bytes = [
                        "R".to_string().as_bytes(),
                        player.clone().as_bytes(),
                        buffer.as_bytes(),
                    ].concat();
                    stream.write(&bytes);
                }
                Package::EndGame {
                    player_1_name, score_1,
                    player_2_name, score_2,
                    player_3_name, score_3,
                    player_4_name, score_4
                } => {
                    println!("Puntajes:");
                    println!("{}: {} puntos", player_1_name, score_1);
                    println!("{}: {} puntos", player_2_name, score_2);
                    println!("{}: {} puntos", player_3_name, score_3);
                    println!("{}: {} puntos", player_4_name, score_4);
                    break;
                }
                Package::Wait { player_id: _ } => {
                    let one_second = time::Duration::from_secs(1);
                    thread::sleep(one_second);
                }
                _ => panic!()
            }
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
