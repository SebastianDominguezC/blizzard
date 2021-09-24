//! # Player
//!
//! The player is what a controller / connector handles as a connection.
//! Each player has a unique id.

#[derive(Serialize, Deserialize, Debug)]
/// Player struct for client connection identification
pub struct Player {
    pub id: usize,
}

impl Player {
    /// Create a new player
    pub fn new(id: usize) -> Player {
        Player { id }
    }
}
