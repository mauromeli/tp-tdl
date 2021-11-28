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
    let server = Server::new();
    server.run(HOST, PORT);
}
