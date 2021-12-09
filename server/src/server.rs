use std::net::TcpListener;
use std::io;
use std::io::Read;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::str;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::client::Client;
use crate::file_reader::reader;
use crate::packages::Package;
use crate::model::kahoot::Kahoot;
use crate::package_handlers::package_handlers;
use crate::package_handlers::package_handlers::CheckStatusRet;

const EXIT_KEY: char = 'q';

type OutChannelSend = Sender<Option<Package>>;
type OutChannelRecv = Receiver<Option<Package>>;

type InChannelSend = Sender<(Package, OutChannelSend)>;
type InChannelRecv = Receiver<(Package, OutChannelSend)>;

// Tipo de dato definido para guardar las conecciones de los usuarios y su estado en uso.
type VecHandler = Vec<(JoinHandle<Result<(), io::Error>>, Arc<AtomicBool>)>;

pub struct Server {
}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub fn run(self, host: &str, port: &str) {
        //Método encargado de spawnear un listener thread por cada cliente
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

        //Hago uso de thread::spawn
        let _handler: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
            let addr = &format!("{}:{}", host_copy, port_copy);
            let listener = TcpListener::bind(addr).unwrap();
            println!("[INFO] - Listening on port {}", port_copy);

            let (in_sender, in_recv): (InChannelSend, InChannelRecv) = mpsc::channel();
            self.spawn_evaluator_thread(in_recv);

            let mut handlers: VecHandler = vec![];
            while let Ok(connection) = listener.accept() {
                let (client_stream, addr) = connection;
                println!("[INFO] - New connection from {}:{}", addr.ip(), addr.port());

                let channel = in_sender.clone();

                let flag = Arc::new(AtomicBool::new(true));
                let used_flag = flag.clone();

                let handler: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
                    let client = Client::new(client_stream, addr);
                    Server::client_handler(client, channel, &used_flag)?;
                    Ok(())
                });

                // Check if we have handlers innactives
                handlers.push((handler, flag));

                let mut handlers_actives: VecHandler = vec![];
                let mut handlers_inactives: VecHandler = vec![];
                for (handler, used) in handlers {
                    if used.load(Ordering::Relaxed) {
                        handlers_actives.push((handler, used));
                    } else {
                        handlers_inactives.push((handler, used));
                    }
                }


                // Join inactives handlers
                for (handler, _) in handlers_inactives {
                    if handler.join().is_err() {
                        println!("Error joining handler")
                    }
                }

                handlers = handlers_actives;
            }

            drop(listener);
            Ok(())
        });
    }

    fn client_handler(mut client: Client, sender: Sender<(Package, Sender<Option<Package>>)>, used: &AtomicBool) -> io::Result<()> {
        let (ret_sender, ret_recv): (OutChannelSend, OutChannelRecv) = mpsc::channel();

        loop {
            let recv_package = client.recv();
            match recv_package {
                Ok(recv_package) => {
                    sender.send((recv_package, ret_sender.clone())).unwrap();

                    let package = ret_recv.recv().unwrap();
                    match package {
                        Some(package) => { client.send(format!("{}", package)); }
                        None => {}
                    }
                }
                Err(..) => {
                    println!("[INFO] - Client {}:{} has disconnected", client.addr.ip(), client.addr.port());
                    break;
                }
            }
        }


        used.swap(false, Ordering::Relaxed);
        Ok(())
    }

    fn spawn_evaluator_thread(self, receiver: Receiver<(Package, Sender<Option<Package>>)>) {
        let _: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
            let questions = reader();
            let mut kahoot = Kahoot::new(questions);

            #[allow(irrefutable_let_patterns)]
            while let (package, sender) = receiver.recv().unwrap() {
                match package {
                    Package::Connect { player_name } => {
                        println!("[INFO] - Se conectó {}", player_name);
                        let player_id = package_handlers::handle_connect_package(&mut kahoot, player_name);
                        sender.send(Some(Package::StartGame { player_id: player_id.to_string() })).unwrap()
                    }
                    Package::CheckStatus { player_id } => {
                        let result = package_handlers::handle_check_status_package(&mut kahoot, player_id.clone());
                        match result {
                            CheckStatusRet::Question { question, options } => {
                                sender.send(Some(Package::Question { question, options }))
                            }
                            CheckStatusRet::End { mut players } => {
                                let players_names: Vec<String> = players.keys().cloned().collect();
                                let players_points: Vec<String> = players.values().cloned().collect();

                                for i in 0..players_names.len() {
                                    players.insert(players_names[i].clone(), players_points[i].clone());
                                }

                                sender.send(Some(Package::EndGame { players }))
                            }
                            CheckStatusRet::Wait {} => {
                                sender.send(Some(Package::Wait { player_id }))
                            }
                        }.unwrap()
                    }
                    Package::Response { player_id, response } => {
                        package_handlers::handle_response_package(&mut kahoot, player_id.clone(), response);
                        sender.send(None).unwrap()
                    }
                    _ => {}
                }
            }
            Ok(())
        });
    }
}
