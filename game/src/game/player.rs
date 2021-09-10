use super::position::Position;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    // #[serde(rename = "lolId")]
    pub id: usize,
    pos: Position,
}

impl Player {
    pub fn new(id: usize) -> Player {
        Player {
            id,
            pos: Position::new(),
        }
    }

    pub fn move_player(&mut self) {
        self.pos.displace_unit();
    }
}
