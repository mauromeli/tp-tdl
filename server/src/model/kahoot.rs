use std::collections::HashMap;
use crate::model::question::Question;
use crate::model::player::Player;

pub struct Kahoot {
    questions: Vec<Question>,
    current_question: u8,
    players: HashMap<u8, Player>
}

impl Kahoot {
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

    pub fn get_players_amount(&self) -> u8 {
        self.players.len() as u8
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
