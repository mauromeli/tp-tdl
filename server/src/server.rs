use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::Read;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use crate::client::Client;
use crate::packages::Package;
use crate::model::kahoot::Kahoot;

const EXIT_KEY: char = 'q';

type ChannelSender = Sender<(String, Sender<String>)>;
type ChannelRecv = Receiver<(String, Sender<String>)>;

pub struct Server {
    pub kahoot_game: Kahoot
}

impl Server {
    pub fn new() -> Server {
        Server {
            kahoot_game: Kahoot::new()
        }
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

            let (in_sender, in_recv): (Sender<(Package, Sender<Package>)>, Receiver<(Package, Sender<Package>)>) = mpsc::channel();
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

    fn client_handler(client: TcpStream, sender: Sender<(Package, Sender<Package>)>) -> io::Result<()> {
        let mut client = Client::new(client);
        let (ret_sender, ret_recv): (Sender<Package>, Receiver<Package>) = mpsc::channel();

        loop {
            let recv_package = client.recv();

            sender.send((recv_package, ret_sender.clone()));

            let package = ret_recv.recv().unwrap();
            println!("{}", format!("{}", package));
            client.send(format!("{}", package));
        }
        Ok(())
    }

    // Probably we can configure this with the answers
    fn spawn_evaluator_thread(mut self, receiver: Receiver<(Package, Sender<Package>)>) {
        let _: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
            let mut var = 1;
            while let (package, sender) = receiver.recv().unwrap() {
                match package {
                    Package::Connect { player_name } => {
                        println!("[INFO] - Se conectó {}", player_name);
                        sender.send(Package::StartGame{ player_id: "1".to_string() });
                    },
                    Package::StartGame { player_id } => {
                        println!("start game");
                        //client.send(&"P¿Cuantos años...?|10 años-200 años-400 años-20 años".to_string());
                    },
                    Package::Response { player_id, response } => {
                        println!("respuesta: {}, player_id: {}", response, player_id);
                    },
                    Package::CheckStatus { player_id } => {
                        // TODO: Delete when kahoot model is connected to server.
                        // Only to swat between WAIT and Answer
                        if var % 2 == 0 {
                            var += 1;
                            sender.send(Package::Wait{ player_id: "1".to_string() });
                        } else {
                            var += 1;
                            sender.send(Package::Question{ question: "¿Quién ganó la Libertadores 2018?".to_string(),
                                options: vec!["River".to_string(), "Boca".to_string(),
                                              "Gremio".to_string(), "Palmeiras".to_string()]
                            });
                        }
                    }
                    _ => {}
                }
            }
            Ok(())
        });
    }
}

//let packet_to_send = packet::command_generator(packet,
//                                             &mut self.kahoot_game);
//sender.send(packet_to_send).unwrap();
