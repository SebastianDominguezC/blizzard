use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::vec::Vec;

#[derive(Debug)]
struct World {
    entity_manager: Vec<Entity>,
    component_manager: HashMap<u32, Vec<Component>>,
}

impl World {
    fn new() -> World {
        World {
            entity_manager: vec![],
            component_manager: HashMap::new(),
        }
    }

    fn update(&mut self) {
        for (_, components) in self.component_manager.iter_mut() {
            components.iter();
            for component in components {
                component.update();
            }
        }
    }

    fn add_entity(&mut self) -> u32 {
        let entity = Entity(self.entity_manager.len() as u32);
        let id = entity.0;
        self.entity_manager.push(entity);
        id
    }

    fn add_components(&mut self, entity: u32, components: Vec<Component>) {
        for component in components {
            self.add_component(entity, component);
        }
    }

    fn add_component(&mut self, entity: u32, component: Component) {
        let entry = self.component_manager.entry(entity).or_insert(vec![]);
        entry.push(component);
    }
}

#[derive(Debug)]
struct Entity(u32);

#[derive(Debug)]
enum Component {
    Type,
}

impl Component {
    fn hi(&self) {
        match self {
            Type => {}
        }
    }
    fn update(&mut self) {}
}

pub fn lol() {}
