use std::collections::HashMap;
use crate::model::kahoot::Kahoot;
use crate::model::player::Player;
use crate::model::question::Question;
use std::option;

#[derive(Debug)]
pub enum CheckStatusRet {
    Question {
        question: String,
        options: Vec<String>
    },
    End {
        players: HashMap<String, String>
    },
    Wait {}
}

pub fn handle_connect_package(game: &mut Kahoot, player_name: String) -> u8 {
    let new_player = Player::new(player_name);
    let id = new_player.id.clone();
    game.add_player(new_player);
    id
}

// Returns (question, options) if there is a new question to player_id
pub fn handle_check_status_package(game: &Kahoot, player_id_str: String) -> CheckStatusRet {
    let player_id = player_id_str.parse::<u8>().unwrap();
    if !game.should_start() || game.player_answered_current_question(player_id) {
        CheckStatusRet::Wait {}
    } else {
        let current_question = game.current_question();
        match current_question {
            Some(question) => {
                CheckStatusRet::Question {
                    question: question.question.clone(),
                    options: question.options.clone() }
            },
            None => {
                // No questions left, match should end
                let mut players = HashMap::new();
                for player in &game.players {
                    players.insert(player.1.name.clone(), player.1.points.to_string());
                }

                CheckStatusRet::End { players }
            }
        }
    }
}

pub fn handle_response_package(game: &mut Kahoot, player_id_str: String, response: String) {
    let player_id = player_id_str.parse::<u8>().unwrap();
    game.answer_current_question(player_id, response);
}
