use std::fmt;

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
    },
    Wait {
        player_id: String,
    },
    Question {
        question: String,
        options: Vec<String>,
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Package::StartGame { player_id } => write!(f, "A{}", player_id),
            Package::Wait { player_id } => write!(f, "H{}", player_id),
            Package::Question { question, options } =>
                write!(f, "P{}|{}-{}-{}-{}", question, options[0], options[1], options[2], options[3]),
            _ => write!(f, "ERROR WHILE FORMATTING PACKET!")
        }
    }
}