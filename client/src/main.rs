mod client;
mod packages;
mod decode_packages;

use crate::client::Client;

const HOST: &str = "localhost";
const PORT: &str = "3004";

fn main() {
    //1) Instancio cliente
    let mut client = Client::new();

    //2) Pongo a correr al cliente
    client.run(HOST, PORT);
}
