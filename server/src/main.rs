mod file_reader;
mod question;
mod client;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;
use crate::client::{Client, Runnable};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:3001").unwrap();
    println!("Listening on port 3001");
    let questions: Vec<question::Question> = file_reader::reader(); //we should create game's class
    println!("{}", questions[0].question);

    for client_stream in listener.incoming() {
        //Now we can have an array of clients
        let mut client = Client::new(client_stream.unwrap());
        Client::run(&mut client, questions.clone());
    }
    drop(listener)
}
