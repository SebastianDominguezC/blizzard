use std::sync::{Arc, Mutex};

pub trait Game<K> {
    fn world_config(&mut self);
    fn update(&mut self, input: i32, shared_state: Arc<Mutex<K>>);
    fn render(&mut self);
    fn end_game(&self) -> bool;
}
