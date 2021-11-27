#[derive(Debug)]
pub enum Package {
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
}
