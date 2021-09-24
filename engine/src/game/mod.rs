use std::sync::{Arc, Mutex};

/// Game definition
pub trait Game<K, I> {
    fn world_config(&mut self);
    fn update(&mut self, input: I, shared_state: Arc<Mutex<K>>);
    fn reset_input(&mut self, input: Arc<Mutex<I>>);
    fn render(&mut self);
    fn end_game(&self) -> bool;
}
