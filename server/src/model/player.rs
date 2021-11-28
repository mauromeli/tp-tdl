use std::cell::Cell;

thread_local!(static NEXT_ID: Cell<u8> = Cell::new(1));

pub struct Player {
    pub id: u8,
    pub points: u32,
    pub name: String,
    pub correct_questions: u8
}

impl Clone for Player{
    fn clone(&self) -> Self {
        return Player {
            id: self.id.clone(),
            name: self.name.clone(),
            points: self.points.clone(),
            correct_questions: self.correct_questions.clone()
        }
    }
}
impl Player {
    pub fn new(name: String) -> Player {
        NEXT_ID.with(|next_id| {
            let id = next_id.get();
            next_id.set(id + 1);
            Player { id, name, points: 0, correct_questions: 0 }
        })
    }

    pub fn add_points(&mut self, points: u32) {
        self.points += points;

        if points > 0 {
            self.correct_questions += 1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::player::Player;

    #[test]
    fn player_id_increases_correctly() {
        let first_player = Player::new("Juan".to_string());
        let second_player = Player::new("Pedro".to_string());
        assert_eq!(first_player.id, 1);
        assert_eq!(second_player.id, 2);
    }
}
