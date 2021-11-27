use std::cell::Cell;

thread_local!(static NEXT_ID: Cell<u8> = Cell::new(1));

pub struct Player {
    pub id: u8,
    pub points: u32,
    pub name: String
}
impl Clone for Player{
    fn clone(&self) -> Self {
        return Player{
            id: self.id.clone(),
            name: self.name.clone(),
            points: self.points.clone()
        }
    }
}
impl Player {
    pub fn new(name: String) -> Player {
        NEXT_ID.with(|next_id| {
            let id = next_id.get();
            next_id.set(id + 1);
            Player { id, name, points: 0 }
        })
    }

    pub fn add_points(&mut self, points: u32) {
        self.points += points;
    }
}

#[cfg(test)]
mod tests {
    use crate::model::player::Player;

    #[test]
    fn player_id_increases_correctly() {
        let first_player = Player::new();
        let second_player = Player::new();
        assert_eq!(first_player.id, 1);
        assert_eq!(second_player.id, 2);
    }
}
