mod client;
mod packages;
mod generate_packages;

use crate::client::Client;
use crate::packages::Package;

const HOST: &str = "localhost";
const PORT: &str = "3004";

fn main() {
    let mut client = Client::new();
    client.run(HOST, PORT);
}
