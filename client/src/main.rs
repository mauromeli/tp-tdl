mod client;
mod packages;
mod decode_packages;

use crate::client::Client;

const HOST: &str = "localhost";
const PORT: &str = "3004";

fn main() {
    //Instancio un cliente
    let mut client = Client::new();

    //Pongo a correr un cliente para que se conecte
    client.run(HOST, PORT);
}
