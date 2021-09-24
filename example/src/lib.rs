#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::ops::AddAssign;

// Message definition
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Message {
    None,
    W,
    A,
    S,
    D,
    AddPlayer,
    RemovePlayer,
}

// Shared state definition
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SharedState {
    pub counters: Vec<u32>,
    pub registry: Vec<Position>,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            registry: vec![],
            counters: vec![],
        }
    }
}

// Position component
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    pub fn displacement(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
