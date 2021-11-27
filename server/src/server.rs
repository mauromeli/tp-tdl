use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::Read;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use crate::client::Client;
use crate::model::kahoot::Kahoot;
use crate::packet;

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

        client.send(&"Mensaje".to_string());

        let recv_string = client.recv();
        println!("Selected option: {}", recv_string);

        let (ch_sender, ch_recv): (Sender<String>, Receiver<String>) = mpsc::channel();
        sender.send((recv_string, ch_sender)).unwrap();

        let response = ch_recv.recv().unwrap();
        client.send(&response);
        Ok(())
    }

    /*
    command_generator() -> Result(Paquete, Err)

	array = C,Mauro
	match array[0] {
		c => connect_generator(array),
		S =>
	}

	connect_generator(array) -> Paquetes {}


	enum Paquetes {
		CONNECT(String: name),
		ACKCONNECT(Sring: id_player),
	}

	paquete = command_generator()

	match(paquete) {
		CONNECT(name) => connectarse_al_juego(name),
		RESPUESTA()
	}
    */


    // Probably we can configure this with the answers
    fn spawn_evaluator_thread(mut self, receiver: ChannelRecv) {
        let _: JoinHandle<Result<(), io::Error>> = thread::spawn(move || {
            while let Ok((option, sender)) = receiver.recv() {
                let mut packet;
                match option.chars().nth(0).unwrap() {
                   'C' => {
                        packet = packet::connect_generator(option);
                    }
                   'R' => {
                        packet = packet::answer_generator(option);
                    }
                   'S' => {
                        packet = packet::score_generator(option);
                    }
                    _ => {
                        packet = packet::error_generator(option);
                    }
                }
                let packet_to_send = packet::command_generator(packet,
                                                               &mut self.kahoot_game);
                sender.send(packet_to_send).unwrap();
            }
            Ok(())
        });
    }
}
