//! # ECS architecture
//!
//! The engine follows an ECS architecture.

use blizzard_id::Uid;
use std::collections::HashMap;
use std::vec::Vec;

/// World definition
/// # Example
/// For a working example, please see official github repo workspace inside the example lib.
pub trait World<I> {
    fn new() -> Self;
    fn run_systems(&mut self, input: I);
}

/// Entity manager
#[derive(Debug, Clone)]
pub struct EntityManager {
    entities: HashMap<u32, bool>,
}

impl EntityManager {
    /// Creates a new entity manager
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    /// Creates entity and returns it's id
    pub fn create_entity(&mut self) -> u32 {
        let mut id = Uid::new_numerical(4);
        if self.entities.contains_key(&id) {
            id = self.create_entity();
        } else {
            self.entities.insert(id, false);
        }
        id
    }

    /// Create multiple entities and return the id's inside a vector
    pub fn create_n_entities(&mut self, n: i32) -> Vec<u32> {
        let mut entities = vec![];
        for _ in 0..n {
            let entity = self.create_entity();
            entities.push(entity);
        }
        entities
    }

    /// Get an entity if it exists
    pub fn get_one(&self, entity: u32) -> Option<u32> {
        match self.entities.get(&entity) {
            Some(_) => Some(entity),
            None => None,
        }
    }

    /// Returns all the entities
    pub fn get_all(&self) -> &HashMap<u32, bool> {
        &self.entities
    }

    /// Mark entity for removal
    pub fn mark_remove(&mut self, entity: u32) {
        self.entities.insert(entity, true);
    }

    /// Remove entity
    pub fn remove_entity(&mut self, entity: u32) {
        self.entities.remove_entry(&entity);
    }

    /// Remove all entities that are marked as remove
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

/// Component registry definition.
/// Macro exists to generate a component on the fly.
pub trait ComponentRegistry<T: Copy> {
    fn new() -> Self;
    fn add(&mut self, entity: u32, component: T);
    fn add_many(&mut self, entities: &Vec<u32>, component: T);
    fn remove(&mut self, entity: u32);
    fn get(&self, entity: u32) -> Option<&T>;
}
