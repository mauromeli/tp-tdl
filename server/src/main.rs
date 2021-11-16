mod file_reader;
mod client;
mod server;
mod model;

use std::str;
use crate::server::Server;

const HOST: &str = "localhost";
const PORT: &str = "3005";

fn main() {
    let server = Server::new();
    server.run(HOST, PORT);
}
