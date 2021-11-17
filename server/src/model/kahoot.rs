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
    const QUESTIONS_TO_ANSWER : u8 = 3;
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

    pub fn get_next_question(&mut self) -> &Question {
        let index: u8 = self.current_question;
        self.current_question += 1;
        self.questions.get(index as usize).unwrap()
    }

    pub fn should_end(&mut self) -> bool{
        if self.current_question == Kahoot::QUESTIONS_TO_ANSWER {
            true
        }else{
            false
        }
    }
    pub fn get_winner(&mut self) -> (u8, u32){
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
        let player: &mut Player = self.players.get_mut(&player_id).expect(PLAYER_NOT_FOUND);
        let current_question : &Question = self.questions.
            get_mut(self.current_question as usize).unwrap();

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
        let player_id_0 = Player::new();
        kahoot.add_player(player_id_0);
        assert_eq!(kahoot.get_players_amount(), 1);
    }

    #[test]
    fn a_player_with_5_points_wins_vs_players_with_less_points() {
        let mut kahoot = Kahoot::new();
        let mut player_id_1 = Player::new();
        let mut player_id_2 = Player::new();
        let mut player_id_3 = Player::new();
        let mut player_id_4 = Player::new();
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
