use std::collections::HashMap;
use crate::model::question::Question;
use crate::model::player::Player;

static PLAYER_NOT_FOUND : &str = "Could not find player";

pub struct Kahoot {
    questions: Vec<Question>,
    current_question: u8,
    pub players: HashMap<u8, Player>,
    players_who_answered: Vec<u8>
}

impl Kahoot {
    const REQUIRED_PLAYERS : u8 = 2;

    pub fn new(questions: Vec<Question>) -> Kahoot {
        Kahoot {
            questions,
            current_question: 0,
            players: HashMap::new(),
            players_who_answered: Vec::new()
        }
    }

    pub fn add_player(&mut self, new_player: Player) {
        self.players.insert(new_player.id, new_player);
    }

    fn get_players_amount(&self) -> u8 {
        self.players.len() as u8
    }

    pub fn should_start(&self) -> bool {
        self.get_players_amount() == Kahoot::REQUIRED_PLAYERS
    }

    pub fn should_end(&self) -> bool {
        self.current_question == self.questions.len() as u8
    }

    // TODO: Remove
    pub fn get_winner(&mut self) -> (u8, u32) {
        //In case of draw, the first player who had been added to the list wins.
        let mut id_winner :u8 = 0;
        let mut points_winner :u32 = 0;
        for (id, player) in self.players.iter() {
            if player.points > points_winner {
                id_winner = id.clone();
                points_winner = player.points.clone();
            }
        }
        (id_winner, points_winner)
    }

    pub fn answer_current_question(&mut self, player_id: u8, option: String) {
        if self.player_answered_current_question(player_id.clone()) || self.should_end() {
            // Players should not answer twice
            // If game ends players can't answer
            return
        }

        let player: &mut Player = self.players.get_mut(&player_id).expect(PLAYER_NOT_FOUND);
        let current_question : &Question = self.questions.
            get_mut(self.current_question as usize).unwrap();

        player.add_points(current_question.get_points_for(option));

        self.players_who_answered.push(player_id);
        if self.players_who_answered.len() as u8 == Kahoot::REQUIRED_PLAYERS {
            // Next question
            self.current_question += 1;
            self.players_who_answered.clear();
        }
    }

    pub fn player_answered_current_question(&self, player_id: u8) -> bool {
        if self.should_end() {
            return false;
        }

        self.players_who_answered.contains(&player_id)
    }

    pub fn current_question(&self) -> Option<&Question> {
        if self.should_end() {
            None
        } else {
            Some(self.questions.get(self.current_question as usize).unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::kahoot::Kahoot;
    use crate::model::player::Player;

    #[test]
    fn a_player_can_be_added() {
        let mut kahoot = Kahoot::new(Vec::new());
        let player_id_0 = Player::new("Juan".to_string());
        kahoot.add_player(player_id_0);
        assert_eq!(kahoot.get_players_amount(), 1);
    }

    #[test]
    fn a_player_with_5_points_wins_vs_players_with_less_points() {
        let mut kahoot = Kahoot::new(Vec::new());
        let mut player_id_1 = Player::new("Juan".to_string());
        let mut player_id_2 = Player::new("Pedro".to_string());
        let mut player_id_3 = Player::new("Pablo".to_string());
        let mut player_id_4 = Player::new("Maria".to_string());
        player_id_1.add_points(0);
        player_id_2.add_points(4);
        player_id_3.add_points(5);
        player_id_4.add_points(3);
        kahoot.add_player(player_id_1.clone());
        kahoot.add_player(player_id_2.clone());
        kahoot.add_player(player_id_3.clone());
        kahoot.add_player(player_id_4.clone());
        assert_eq!(kahoot.get_winner(), (3,5));
    }
}
