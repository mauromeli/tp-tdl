mod client;

use crate::client::Client;

const HOST: &str = "localhost";
const PORT: &str = "3000";

fn main() {
    let mut client = Client::new();
    client.run(HOST, PORT);
}
