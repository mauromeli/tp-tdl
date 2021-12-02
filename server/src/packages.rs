use std::collections::HashMap;
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
    },
    EndGame {
        players: HashMap<String, String>,
    }
/*
    EndGame {
        player_1_name: String,
        score_1: String,
        player_2_name: String,
        score_2: String,
        player_3_name: String,
        score_3: String,
        player_4_name: String,
        score_4: String
    }

 */
}
impl std::fmt::Display for Package {


    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            Package::StartGame { player_id } => write!(f, "A{}", player_id),
            Package::Wait { player_id } => write!(f, "W{}", player_id),
            Package::Question { question, options } =>
                write!(f, "P{}|{}-{}-{}-{}", question, options[0], options[1], options[2], options[3]),
            Package::EndGame { players } => {
                write!(f, "E").unwrap();
                let mut i = 1;
                for (key, value) in players {
                    write!(f, "{},{}", key, value).unwrap();
                    if i < players.len() {
                        write!(f, ",").unwrap();
                    }
                    i = i + 1
                }
                write!(f, "")
            },
            _ => write!(f, "ERROR WHILE FORMATTING PACKET!")
        }
    }
}