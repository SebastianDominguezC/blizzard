use std::collections::HashMap;
use std::vec::Vec;
use uid::Uid;

pub trait World<I> {
    fn new() -> Self;
    fn run_systems(&mut self, input: I);
}

// Entity definition
#[derive(Debug, Clone)]
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

    pub fn create_n_entities(&mut self, n: i32) -> Vec<u32> {
        let mut entities = vec![];
        for _ in 0..n {
            let entity = self.create_entity();
            entities.push(entity);
        }
        entities
    }

    pub fn get_one(&self, entity: u32) -> Option<u32> {
        match self.entities.get(&entity) {
            Some(_) => Some(entity),
            None => None,
        }
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
pub trait ComponentRegistry<T: Copy> {
    fn new() -> Self;
    fn add(&mut self, entity: u32, component: T);
    fn add_many(&mut self, entities: &Vec<u32>, component: T);
    fn remove(&mut self, entity: u32);
    fn get(&self, entity: u32) -> Option<&T>;
}
