use std::collections::HashMap;
use std::vec::Vec;
use uid::Uid;

pub trait World {
    fn new() -> Self;
    fn run_systems(&mut self);
}

// Entity definition
#[derive(Debug)]
pub struct EntityManager {
    entities: HashMap<u32, bool>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> u32 {
        let mut id = Uid::new_numerical(4);
        if self.entities.contains_key(&id) {
            id = self.create_entity();
        } else {
            self.entities.insert(id, false);
        }
        id
    }

    pub fn get_all(&self) -> &HashMap<u32, bool> {
        &self.entities
    }

    pub fn mark_remove(&mut self, entity: u32) {
        self.entities.insert(entity, true);
    }

    fn remove_entity(&mut self, entity: u32) {
        self.entities.remove_entry(&entity);
    }

    pub fn remove_entities(&mut self) {
        let entities: Vec<u32> = self
            .entities
            .iter()
            .filter(|(_, remove)| **remove)
            .map(|(id, _)| *id)
            .collect();
        entities.iter().for_each(|id| {
            self.remove_entity(*id);
        });
    }
}

// Component definition
pub trait ComponentRegistry<T> {
    fn new() -> Self;
    fn add(&mut self, entity: u32, component: T);
    fn remove(&mut self, entity: u32);
    fn get(&self, entity: u32) -> Option<&T>;
}

// System definition
pub trait System<T> {
    fn new() -> Self;
    fn update(&self, components: &mut HashMap<u32, T>);
}
