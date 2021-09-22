use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::vec::Vec;

#[derive(Debug)]
pub struct World {
    pub entity_manager: Vec<Entity>,
    pub component_manager: HashMap<u32, Vec<Box<dyn Component>>>,
}

impl World {
    pub fn new() -> World {
        World {
            entity_manager: vec![],
            component_manager: HashMap::new(),
        }
    }

    pub fn update(&mut self, input: u32) {
        for (_, components) in self.component_manager.iter_mut() {
            for component in components {
                component.update(input);
            }
        }
    }

    pub fn add_entity(&mut self) -> u32 {
        let entity = Entity(self.entity_manager.len() as u32);
        let id = entity.0;
        self.entity_manager.push(entity);
        id
    }

    pub fn add_components(&mut self, entity: u32, components: Vec<Box<dyn 'static + Component>>) {
        for component in components {
            self.add_component(entity, component);
        }
    }

    pub fn add_component(&mut self, entity: u32, component: Box<dyn 'static + Component>) {
        let entry = self.component_manager.entry(entity).or_insert(vec![]);
        entry.push(component);
    }

    pub fn query(&self, query: Query) -> Vec<u32> {
        let mut entities = vec![];
        for (id, components) in self.component_manager.iter() {
            // Turn components into types
            let ctypes: Vec<Type> = components
                .iter()
                .map(|component| component.ctype())
                .collect();

            // Transform types into vectors of bools
            // Fold vectors into a single bool
            let query = query
                .0
                .iter()
                .map(|ctype| ctypes.contains(ctype))
                .fold(true, |acc, b| acc && b);

            if query {
                entities.push(*id);
            }
        }
        return entities;
    }

    fn run_system(&self, query: Query) {
        let entities = self.query(query);
        for entity in entities.iter() {
            // self.component_manager[entity] =
        }
    }
}

#[derive(Debug)]
pub struct Entity(pub u32);

#[derive(PartialEq, Debug)]
pub enum Type {
    Default,
    Other,
}

impl Type {
    fn default() -> Type {
        Type::Default
    }
}

pub trait Component {
    fn update(&mut self, input: u32);
    fn print(&self) -> String;
    fn component_name(&self) -> String;
    fn ctype(&self) -> Type {
        Type::default()
    }
}

impl Debug for dyn Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&self.component_name())
            .field("Data", &self.print())
            .finish()
    }
}

pub struct Query(Vec<Type>);

impl Query {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn from(types: Vec<Type>) -> Self {
        Self(types)
    }
    // pub fn map(&self) -> Vec<Box<dyn Component>> {
    //     self.0.iter().map(|t| {
    //         match t {
    //             Type::Default =>
    //         }
    //     }).collect();
    // }
}

trait System {
    fn run_system(query: Query);
}

struct OtherSystem {}

impl System for OtherSystem {
    fn run_system(query: Query) {}
}
