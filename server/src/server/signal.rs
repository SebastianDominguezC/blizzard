use crate::game::Position;

#[derive(Serialize, Deserialize, Debug)]
pub enum Signal {
    MovePlayer(Position),
    StartConnection,
    EndConnection,
}

impl Signal {
    pub fn default_move() -> Signal {
        Signal::MovePlayer(Position::new())
    }
}
