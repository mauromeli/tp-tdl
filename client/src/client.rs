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

    pub fn run(&self, host: &str, port: &str) {
        let player_name = self.name_consultor();

        let addr = &format!("{}:{}", host, port);

        //1) Conexion TCP a un Host remoto
        let mut stream = TcpStream::connect(addr).unwrap();

        let bytes = [
            "C".to_string().as_bytes(),
            player_name.as_bytes(),
        ].concat();

        //2) Coloco en el stream todos los datos a enviar
        stream.write_all(&bytes).unwrap();

        //Nuevo buffer inicializado con 0
        let mut recv_buffer = [0; 1024];

        //3) Recibo datos del stream correspondiente
        let mut bytes_amount_received = stream.read(&mut recv_buffer).unwrap();

        let package_to_decode = &recv_buffer[0..bytes_amount_received];

        let ack_package = decode_package(package_to_decode).unwrap();
        let player: String;

        match ack_package {
            Package::ACKConnect { player_id } => {
                println!("Esperando más jugadores...");
                player = player_id;
            }
            _ => {
                println!("No me pude conectar");
                return;
            }
        }

        loop {
            let bytes = [
                "W".to_string().as_bytes(), //Check Status
                player.clone().as_bytes(),
            ].concat();

            //4)
            stream.write_all(&bytes).unwrap();

            //Nuevo buffer inicializado con 0
            let mut recv_buffer = [0; 1024];

            //5)
            bytes_amount_received = stream.read(&mut recv_buffer).unwrap();

            let package_to_decode = &recv_buffer[0..bytes_amount_received];

            let package = decode_package(package_to_decode).unwrap();

            match package {
                Package::Question { question, options } => {
                    println!("\nPregunta: {}", question);
                    println!("A) {}", options[0]);
                    println!("B) {}", options[1]);
                    println!("C) {}", options[2]);
                    println!("D) {}", options[3]);

                    let mut buffer = String::new();
                    io::stdin().read_line(&mut buffer).unwrap();
                    buffer.pop(); // Remove newline

                    let bytes = [
                        "R".to_string().as_bytes(),
                        player.clone().as_bytes(),
                        buffer.as_bytes(),
                    ].concat();

                    //6) Coloco en el stream los datos a enviar
                    stream.write_all(&bytes).unwrap();

                    let one_second = time::Duration::from_secs(1);
                    thread::sleep(one_second);
                }
                Package::EndGame {
                    players
                } => {
                    let mut sorted_players: Vec<_> = players.iter().collect();
                    sorted_players.sort_by_key(|a| a.0);

                    println!("\nTabla de puntajes:");
                    for (key, value) in sorted_players.iter() {
                        println!("{}: {} puntos", key, value);
                    }
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
        println!("Bienvenido a Kaho-rust! ¿Cuál es tu nombre?");

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap(); // <- API requires buffer param as of Rust 1.0; returns `Result` of bytes read
        let res = match buffer.trim_end() {
            "" => {
                let name = self.name_generator();
                println!("Oh! veo que sos de pocas palabras. Te llamaremos {}", name);
                name
            }
            name => name.to_string(),
        };

        println!("Hola {}. Mucha suerte :)", res);
        res
    }

    fn name_generator(&self) -> String {
        "BOT".to_string()
    }
}
