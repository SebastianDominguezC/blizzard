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
            components.iter();
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
}

#[derive(Debug)]
pub struct Entity(pub u32);

pub trait Component {
    fn update(&mut self, input: u32);
    fn print(&self) -> String;
    fn component_name(&self) -> String;
}

impl Debug for dyn Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&self.component_name())
            .field("Data", &self.print())
            .finish()
    }
}
