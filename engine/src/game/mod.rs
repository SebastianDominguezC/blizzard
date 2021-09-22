pub trait Game {
    fn world_config(&mut self);
    fn update(&mut self, input: u32);
    fn render(&mut self);
    fn end_game(&self) -> bool;
}
