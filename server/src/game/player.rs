#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    // #[serde(rename = "lolId")]
    pub id: usize,
}

impl Player {
    pub fn new(id: usize) -> Player {
        Player { id }
    }
}
