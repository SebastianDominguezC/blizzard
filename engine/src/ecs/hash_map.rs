use std::collections::HashMap;
use std::vec::Vec;
use uid::Uid;

pub fn run() {
    println!("___ RUNNING OTHER ____");
    let mut my_world = MyWorld::new();
    let ent1 = my_world.entity_manager.create_entity();
    my_world.position.add(ent1, Position::new());

    println!("{:?}", my_world.entity_manager.get_all());
    my_world.run_systems();

    println!("{:?}", my_world.position);
    my_world.entity_manager.mark_remove(ent1);
    my_world.entity_manager.remove_entities();

    my_world.run_systems();
    println!("{:?}", my_world.entity_manager.get_all());
}

struct MyWorld {
    entity_manager: EntityManager,
    position: PositionRegistry,
}

impl World for MyWorld {
    fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            position: PositionRegistry::new(),
        }
    }
    fn run_systems(&mut self) {
        PositionSystem::new().update(&mut self.position.positions);
    }
}

trait World {
    fn new() -> Self;
    fn run_systems(&mut self);
}

// Entity definition
pub struct EntityManager {
    entities: HashMap<u32, bool>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    fn create_entity(&mut self) -> u32 {
        let mut id = Uid::new_numerical(4);
        if self.entities.contains_key(&id) {
            id = self.create_entity();
        } else {
            self.entities.insert(id, false);
        }
        id
    }

    fn get_all(&self) -> &HashMap<u32, bool> {
        &self.entities
    }

    fn mark_remove(&mut self, entity: u32) {
        self.entities.insert(entity, true);
    }

    fn remove_entity(&mut self, entity: u32) {
        self.entities.remove_entry(&entity);
    }

    fn remove_entities(&mut self) {
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
trait ComponentRegistry<T> {
    fn new() -> Self;
    fn add(&mut self, entity: u32, component: T);
    fn remove(&mut self, entity: u32);
    fn get(&self, entity: u32) -> Option<&T>;
}

// System definition
trait System<T> {
    fn new() -> Self;
    fn update(&self, components: &mut HashMap<u32, T>);
}

// Example
#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    fn displace(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

#[derive(Debug)]
struct PositionRegistry {
    positions: HashMap<u32, Position>,
}

impl ComponentRegistry<Position> for PositionRegistry {
    fn new() -> Self {
        Self {
            positions: HashMap::new(),
        }
    }
    fn add(&mut self, entity: u32, position: Position) {
        self.positions.insert(entity, position);
    }
    fn remove(&mut self, entity: u32) {
        self.positions.remove(&entity);
    }
    fn get(&self, entity: u32) -> Option<&Position> {
        self.positions.get(&entity)
    }
}

struct PositionSystem();

impl System<Position> for PositionSystem {
    fn new() -> Self {
        Self()
    }
    fn update(&self, positions: &mut HashMap<u32, Position>) {
        for (_, position) in positions.iter_mut() {
            position.displace(1, 1);
        }
    }
}
