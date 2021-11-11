use std::net::TcpListener;
use crate::{file_reader, question};
use crate::client::Client;

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub fn run(&mut self, host: &str, port: &str) {
        let addr = &format!("{}:{}", host, port);
        let listener = TcpListener::bind(addr).unwrap();
        println!("Listening on port {}", port);

        let questions: Vec<question::Question> = file_reader::reader(); //we should create game's class

        for client_stream in listener.incoming() {
            //Now we can have an array of clients
            let mut client = Client::new(client_stream.unwrap());
            client.send(&questions[0].question);
            let recv_string = client.recv();
            println!("Selected option: {}", recv_string);
        }

        drop(listener)
    }
}
