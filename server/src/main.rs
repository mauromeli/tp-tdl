mod file_reader;
mod question;
mod client;

use std::net::TcpListener;
use std::str;
use crate::client::{Client, Runnable};

const HOST: &str = "localhost";
const PORT: &str = "3000";

fn main() {
    let addr = &format!("{}:{}", HOST, PORT);
    let listener = TcpListener::bind(addr).unwrap();
    println!("Listening on port {}", PORT);

    let questions: Vec<question::Question> = file_reader::reader(); //we should create game's class

    for client_stream in listener.incoming() {
        //Now we can have an array of clients
        let mut client = Client::new(client_stream.unwrap());
        Client::send(&mut client, &questions[0].question);
        let recv_string = Client::recv(&mut client);
        println!("Selected option: {}", recv_string);
    }

    drop(listener)
}
