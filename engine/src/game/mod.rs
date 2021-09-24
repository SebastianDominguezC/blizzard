use std::sync::{Arc, Mutex};

pub trait Game<K, I> {
    fn world_config(&mut self);
    fn update(&mut self, input: I, shared_state: Arc<Mutex<K>>);
    fn render(&mut self);
    fn end_game(&self) -> bool;
}
