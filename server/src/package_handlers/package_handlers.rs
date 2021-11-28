use crate::model::kahoot::Kahoot;
use crate::model::player::Player;
use crate::model::question::Question;
use std::option;

pub fn handle_connect_package(game: &mut Kahoot, player_name: String) -> u8 {
    let new_player = Player::new(player_name);
    let id = new_player.id.clone();
    game.add_player(new_player);
    id
}

// Returns (question, options) if there is a new question to player_id
pub fn handle_check_status_package(game: &Kahoot, player_id_str: String) -> Option<(String, Vec<String>)> {
    let player_id = player_id_str.parse::<u8>().unwrap();
    if !game.should_start() || game.player_answered_current_question(player_id) {
        None
    } else {
        let current_question = game.current_question();
        Some((current_question.question.clone(), current_question.options.clone()))
    }
}
