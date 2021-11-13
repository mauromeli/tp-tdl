use std::net::{TcpListener, TcpStream};
use std::io;
use std::thread;
use std::thread::JoinHandle;
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

        while let Ok(connection) = listener.accept() {
            //Now we can have an array of clients
            let (client_stream, _) = connection;

            //we should create some shared structure, maybe could be a mutex or a channels solution
            let question_cloned = questions.clone();
            let _handler: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
                Server::client_handler(client_stream, question_cloned)?;
                Ok(())
            });
        }

        drop(listener)
    }

    fn client_handler(client: TcpStream, questions: Vec<question::Question>) -> io::Result<()> {

        let mut client = Client::new(client);
        client.send(&questions[0].question);
        let recv_string = client.recv();
        println!("Selected option: {}", recv_string);

        Ok(())
    }
}
