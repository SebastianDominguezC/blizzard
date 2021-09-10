#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new() -> Self {
        Position { x: 0, y: 0 }
    }

    pub fn displace_unit(&mut self) {
        self.x += 1;
        self.y += 1;
    }
}
