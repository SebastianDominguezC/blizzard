use std::collections::HashMap;

#[derive(Debug)]
struct World {
    entity_manager: Vec<Entity>,
    component_manager: HashMap<u32, Vec<SomeComponent>>,
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

    // fn add_component<'a, T>(&mut self, entity: u32, component:Box<dyn 'a +  Component>)
    fn add_component(&mut self, entity: u32, component: SomeComponent) {
        let entry = self.component_manager.entry(entity).or_insert(vec![]);
        entry.push(component);
    }
}

#[derive(Debug)]
struct Entity(u32);

trait Component {
    fn update(&mut self);
}

#[derive(Copy, Clone, Debug)]
struct SomeComponent {
    count: i32,
}

impl Component for SomeComponent {
    fn update(&mut self) {
        self.count += 1;
    }
}

pub fn lol() {
    let mut world = World::new();
    let ent1 = world.add_entity();
    let component = SomeComponent { count: 0 };
    world.add_component(ent1, component);
    world.update();
    println!("{:?}", world);
}
