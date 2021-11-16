use std::collections::HashMap;
use crate::model::question::Question;
use crate::model::player::Player;

static PLAYER_NOT_FOUND : &str = "Could not find player";

pub struct Kahoot {
    questions: Vec<Question>,
    current_question: u8,
    players: HashMap<u8, Player>
}

impl Kahoot {
    const REQUIRED_PLAYERS : u8 = 4;

    pub fn new() -> Kahoot {
        Kahoot {
            questions: Vec::new(),
            current_question: 0,
            players: HashMap::new()
        }
    }

    pub fn add_player(&mut self, new_player: Player) {
        self.players.insert(new_player.id, new_player);
    }

    fn get_players_amount(&self) -> u8 {
        self.players.len() as u8
    }

    pub fn add_questions(&mut self, questions: Vec<Question>) {
        self.questions = questions;
    }

    pub fn should_start(&self) -> bool {
        self.get_players_amount() == Kahoot::REQUIRED_PLAYERS
    }

    pub fn answer_current_question(&mut self, player_id: u8, option: String) {
        let player: &mut Player = self.players.get_mut(&player_id).expect(PLAYER_NOT_FOUND);
        let current_question : &Question = self.questions.get(self.current_question as usize).unwrap();

        let player_prev_points : u32 = player.points;
        player.add_points(current_question.get_points_for(option));

        if player_prev_points < player.points {
            // Player answered correctly
        } else {
            // Player answered incorrectly
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::kahoot::Kahoot;
    use crate::model::player::Player;

    #[test]
    fn a_player_can_be_added() {
        let mut kahoot = Kahoot::new();
        let player = Player::new();
        kahoot.add_player(player);
        assert_eq!(kahoot.get_players_amount(), 1);
    }
}
