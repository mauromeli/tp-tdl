use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::Read;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::str;
use crate::client::Client;
use crate::packages::Package;
use crate::model::kahoot::Kahoot;
use crate::package_handlers::package_handlers;
use crate::model::question::Question;

const EXIT_KEY: char = 'q';

type ChannelSender = Sender<(String, Sender<String>)>;
type ChannelRecv = Receiver<(String, Sender<String>)>;

pub struct Server {
    // pub kahoot_game: Kahoot
}

impl Server {
    pub fn new() -> Server {
        Server {
            // kahoot_game: Kahoot::new()
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
            // TODO: Read questions from file
            let options = vec!["River".to_string(), "Boca".to_string(),
                               "Gremio".to_string(), "Palmeiras".to_string()];
            let question = Question::new("¿Quién ganó la Libertadores 2018?".to_string(),
                                         options,
                                         "River".to_string());

            let mut kahoot = Kahoot::new(vec![question]);
            while let (package, sender) = receiver.recv().unwrap() {
                match package {
                    Package::Connect { player_name } => {
                        println!("[INFO] - Se conectó {}", player_name);
                        let player_id = package_handlers::handle_connect_package(&mut kahoot, player_name);
                        sender.send(Package::StartGame{ player_id: player_id.to_string() });
                    },
                    Package::CheckStatus { player_id } => {
                        let result = package_handlers::handle_check_status_package(&kahoot, player_id.clone());
                        match result {
                            // There is a new question for player_id
                            Some((question, options)) => sender.send(Package::Question{ question, options }),
                            // Wait
                            None => sender.send(Package::Wait{ player_id }),
                        };
                    },
                    Package::Response { player_id, response } => {
                        println!("respuesta: {}, player_id: {}", response, player_id);
                    }
                    _ => {}
                }
            }
            Ok(())
        });
    }
}
