mod file_reader;
mod client;
mod server;
mod model;
mod packages;
mod package_handlers;

use std::str;
use crate::server::Server;

const HOST: &str = "localhost";
const PORT: &str = "3004";

fn main() {
    //Instancio el servidor
    let server = Server::new();

    //Pongo a correr el servidor en HOST; PORT
    server.run(HOST, PORT);
}
