use std::ops::{Add, AddAssign};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new() -> Self {
        Position { x: 0, y: 0 }
    }

    pub fn direction(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn up() -> Self {
        Position::direction(0, 1)
    }
    pub fn down() -> Self {
        Position::direction(0, -1)
    }
    pub fn left() -> Self {
        Position::direction(-1, 0)
    }
    pub fn right() -> Self {
        Position::direction(1, 0)
    }

    pub fn displace_unit(&mut self) {
        self.displace(1, 1);
    }

    pub fn displace(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
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
