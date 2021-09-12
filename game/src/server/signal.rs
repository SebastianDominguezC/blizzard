
#[derive(Serialize, Deserialize, Debug)]
pub enum Signal {
    MovePlayer(),
    StartConnection,
    EndConnection,
}
