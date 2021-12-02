use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::Read;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::str;
use crate::client::Client;
use crate::file_reader::reader;
use crate::packages::Package;
use crate::model::kahoot::Kahoot;
use crate::package_handlers::package_handlers;
use crate::model::question::Question;
use crate::package_handlers::package_handlers::CheckStatusRet;

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

            let (in_sender, in_recv): (Sender<(Package, Sender<Option<Package>>)>, Receiver<(Package, Sender<Option<Package>>)>) = mpsc::channel();
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

    fn client_handler(client: TcpStream, sender: Sender<(Package, Sender<Option<Package>>)>) -> io::Result<()> {
        let mut client = Client::new(client);
        let (ret_sender, ret_recv): (Sender<Option<Package>>, Receiver<Option<Package>>) = mpsc::channel();

        loop {
            let recv_package = client.recv();

            sender.send((recv_package, ret_sender.clone()));

            let package = ret_recv.recv().unwrap();
            match package {
                Some(package) => { client.send(format!("{}", package)); }
                None => {}
            }
        }
    }

    // Probably we can configure this with the answers
    fn spawn_evaluator_thread(mut self, receiver: Receiver<(Package, Sender<Option<Package>>)>) {
        let _: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
            let questions = reader();
            let mut kahoot = Kahoot::new(questions);
            while let (package, sender) = receiver.recv().unwrap() {
                match package {
                    Package::Connect { player_name } => {
                        println!("[INFO] - Se conectó {}", player_name);
                        let player_id = package_handlers::handle_connect_package(&mut kahoot, player_name);
                        sender.send(Some(Package::StartGame{ player_id: player_id.to_string() }));
                    },
                    Package::CheckStatus { player_id } => {
                        let result = package_handlers::handle_check_status_package(&kahoot, player_id.clone());
                        match result {
                            CheckStatusRet::Question { question, options } => {
                                sender.send(Some(Package::Question{ question, options }))
                            },
                            CheckStatusRet::End { players } => {
                                // TODO: Correct this
                                let players_names : Vec<String> = players.keys().cloned().collect();
                                let players_points : Vec<String> = players.values().cloned().collect();
                                sender.send(Some(Package::EndGame{
                                    player_1_name: players_names[0].clone(), score_1: players_points[0].clone(),
                                    player_2_name: "Empty".to_string(), score_2: "0".to_string(),
                                    player_3_name: "Empty".to_string(), score_3: "0".to_string(),
                                    player_4_name: "Empty".to_string(), score_4: "0".to_string()
                                }))
                            },
                            CheckStatusRet::Wait {} => {
                                sender.send(Some(Package::Wait{ player_id }))
                            }
                        };
                    },
                    Package::Response { player_id, response } => {
                        println!("Respuesta: {}, player_id: {}", response, player_id);
                        package_handlers::handle_response_package(&mut kahoot, player_id.clone(), response);
                        sender.send(None);
                    }
                    _ => {}
                }
            }
            Ok(())
        });
    }
}
