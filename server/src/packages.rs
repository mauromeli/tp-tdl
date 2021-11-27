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
    }/*,
    ACKConnect {
        player_id: String,
    },
    Question {
        question: String,
        options: Vec<String>,
    },
    FinishGame {
        player_id: String,
    }
    */
}
