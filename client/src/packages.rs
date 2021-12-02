use std::collections::HashMap;
#[derive(Debug)]
pub enum Package {
    ACKConnect {
        player_id: String,
    },
    Question {
        question: String,
        options: Vec<String>,
    },
    EndGame {
        players: HashMap<String, String>,
    },
    Wait {
        player_id: String,
    }
}
