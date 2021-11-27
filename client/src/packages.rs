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
        player_1_name: String,
        score_1: String,
        player_2_name: String,
        score_2: String,
        player_3_name: String,
        score_3: String,
        player_4_name: String,
        score_4: String,
    },
    Wait {
        player_id: String,
    }
}
