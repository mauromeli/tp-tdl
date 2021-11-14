use std::net::{TcpListener, TcpStream};
use std::io;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use crate::{file_reader, question};
use crate::client::Client;


type ChannelSender = Sender<(String, Sender<String>)>;
type ChannelRecv = Receiver<(String, Sender<String>)>;

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub fn run(mut self, host: &str, port: &str) {
        let addr = &format!("{}:{}", host, port);
        let listener = TcpListener::bind(addr).unwrap();
        println!("Listening on port {}", port);

        let questions: Vec<question::Question> = file_reader::reader(); //we should create game's class

        let (chSender, chRecv): (ChannelSender, ChannelRecv) = mpsc::channel();
        self.evaluator_thread(chRecv);

        while let Ok(connection) = listener.accept() {
            //Now we can have an array of clients
            let (client_stream, _) = connection;
            let channel = chSender.clone();

            //we should create some shared structure, maybe could be a mutex or a channels solution
            let question_cloned = questions.clone();
            let _handler: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
                Server::client_handler(client_stream, question_cloned, channel)?;
                Ok(())
            });
        }

        drop(listener)
    }

    fn client_handler(client: TcpStream, questions: Vec<question::Question>, sender: ChannelSender) -> io::Result<()> {
        let mut client = Client::new(client);
        client.send(&questions[0].question);
        let recv_string = client.recv();
        println!("Selected option: {}", recv_string);
        let (chSender, chRecv): (Sender<String>, Receiver<String>) = mpsc::channel();
        sender.send((recv_string, chSender)).unwrap();

        let response = chRecv.recv().unwrap();
        client.send(&response);
        Ok(())
    }

    // Probably we can configure this with the answers
    fn evaluator_thread(self, receiver: ChannelRecv) {
        let _: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
            while let Ok((opcion, sender)) = receiver.recv() {
                match opcion.as_str() {
                    "a" => {
                        sender.send("Correcto".to_string())
                    }
                    _ => {
                        sender.send("incorrecto".to_string())
                    }
                }
            }
            Ok(())
        });
    }
}
