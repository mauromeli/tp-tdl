pub enum Package {
    Connect {
        player_name: String,
    },
    ACKConnect {
        player_id: String,
    },
    Response {
        player_id: String,
        response: String,
    },
    Question {
        question: String,
        options: Vec<String>,
    },
    FinishGame {
        player_id: String,
    }
}

