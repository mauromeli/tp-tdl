use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::Read;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use crate::client::Client;
use crate::packages::Package;

const EXIT_KEY: char = 'q';

type ChannelSender = Sender<(String, Sender<String>)>;
type ChannelRecv = Receiver<(String, Sender<String>)>;

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub fn run(self, host: &str, port: &str) {
        self.spawn_listener_thread(host, port);

        for byte in io::stdin().bytes() {
            let c = byte.unwrap() as char;
            if c == EXIT_KEY {
                break;
            }
        }
    }

    fn spawn_listener_thread(self, host: &str, port: &str) {
        let host_copy = host.to_string();
        let port_copy = port.to_string();

        let _handler: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
            let addr = &format!("{}:{}", host_copy, port_copy);
            let listener = TcpListener::bind(addr).unwrap();
            println!("[INFO] - Listening on port {}", port_copy);

            // FIXME: This should not be here
            // let questions: Vec<question::Question> = file_reader::reader(); //we should create game's class

            let (in_sender, in_recv): (ChannelSender, ChannelRecv) = mpsc::channel();
            self.spawn_evaluator_thread(in_recv);

            while let Ok(connection) = listener.accept() {
                // Now we can have an array of clients
                let (client_stream, addr) = connection;
                println!("[INFO] - New connection from {}:{}", addr.ip(), addr.port());

                // We should create some shared structure, maybe could be a mutex or a channels solution
                let channel = in_sender.clone();
                // let question_cloned = questions.clone();
                let _handler: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
                    Server::client_handler(client_stream, channel)?;
                    Ok(())
                });
            }

            drop(listener);
            Ok(())
        });
    }

    fn client_handler(client: TcpStream, sender: ChannelSender) -> io::Result<()> {
        let mut client = Client::new(client);

        while let package = client.recv() {
            match package {
                Package::Connect { player_name } => client.send(&"ackconnect".to_string()),
                Package::StartGame { player_id } => {
                    println!("start game");
                    client.send(&"pregunta".to_string())
                },
                Package::Response { player_id, response } => {
                    println!("respuesta: {}, player_id: {}", response, player_id);
                    client.send(&"correcto - pregunta".to_string());
                    //client.send(&"pregunta".to_string())
                }
            }
        }



        //println!("Selected option: {}", recv_string);
        /*
        let (ch_sender, ch_recv): (Sender<String>, Receiver<String>) = mpsc::channel();
        sender.send((recv_string, ch_sender)).unwrap();

        let response = ch_recv.recv().unwrap();
        client.send(&response);
        */
        Ok(())
    }

    // Probably we can configure this with the answers
    fn spawn_evaluator_thread(self, receiver: ChannelRecv) {
        let _: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
            while let Ok((opcion, sender)) = receiver.recv() {
                match opcion.as_str() {
                    "a" => {
                        sender.send("Correcto".to_string()).unwrap();
                    }
                    _ => {
                        sender.send("Incorrecto".to_string()).unwrap();
                    }
                }
            }
            Ok(())
        });
    }

    fn connect_client(client: TcpStream, sender: ChannelSender) -> Client {
        let mut client = Client::new(client);

        client.send(&"Ingrese su nombre de usuario".to_string());

        let recv_string = client.recv();
        println!("Nombre de usuario: {:?}", recv_string);

        return client;
    }
}
