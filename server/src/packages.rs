#[derive(Debug)]
pub enum Package {
    Connect {
        player_name: String,
    },
    StartGame {
        player_id: String,
    },
    Response {
        player_id: String,
        response: String,
    },
    CheckStatus {
        player_id: String,
    }
}
