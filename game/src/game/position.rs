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
        self.displace(1, 1);
    }

    pub fn displace(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}
