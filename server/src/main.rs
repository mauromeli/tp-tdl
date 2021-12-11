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
    //1)
    let server = Server::new();

    //2)
    server.run(HOST, PORT);
}
