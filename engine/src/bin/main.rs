extern crate blizzard_engine;
use blizzard_engine::ecs::{ComponentRegistry, EntityManager, World};
use blizzard_engine::game::Game;
use std::collections::HashMap;

// World
#[derive(Debug)]
struct MyWorld {
    entity_manager: EntityManager,
    position: PositionRegistry,
    counters: CounterRegistry,
}

impl World for MyWorld {
    fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            position: PositionRegistry::new(),
            counters: CounterRegistry::new(),
        }
    }
    fn run_systems(&mut self) {
        position_system(&mut self.position.positions);
        counter_system(&mut self.counters.counters)
    }
}

// Components
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
struct CounterRegistry {
    counters: HashMap<u32, i32>,
}

impl ComponentRegistry<i32> for CounterRegistry {
    fn new() -> Self {
        Self {
            counters: HashMap::new(),
        }
    }
    fn add(&mut self, entity: u32, counter: i32) {
        self.counters.insert(entity, counter);
    }
    fn remove(&mut self, entity: u32) {
        self.counters.remove(&entity);
    }
    fn get(&self, entity: u32) -> Option<&i32> {
        self.counters.get(&entity)
    }
}

// Systems
fn position_system(positions: &mut HashMap<u32, Position>) {
    for (_, position) in positions.iter_mut() {
        position.displace(1, 1);
    }
}

fn counter_system(counters: &mut HashMap<u32, i32>) {
    for (_, counter) in counters.iter_mut() {
        *counter += 1;
    }
}

// Games
struct MyGame {
    world: MyWorld,
    counter: u32,
}

impl Game for MyGame {
    fn world_config(&mut self) {
        // Create Entities
        let ent1 = self.world.entity_manager.create_entity();
        let ent2 = self.world.entity_manager.create_entity();
        let ent3 = self.world.entity_manager.create_entity();

        // Add components
        self.world.position.add(ent1, Position::new());
        self.world.counters.add(ent2, 1);
        self.world.position.add(ent3, Position::new());
        self.world.counters.add(ent3, 0);
    }

    fn update(&mut self, input: u32) {
        self.world.run_systems();
        self.counter += 1;
        println!("{:#?}", self.world);
    }

    fn render(&mut self) {}

    fn end_game(&self) -> bool {
        self.counter > 10
    }
}

fn new_game(world: MyWorld) -> impl Game {
    MyGame {
        counter: 0,
        world: world,
    }
}

fn main() {
    let game = new_game(MyWorld::new());
    blizzard_engine::start(game);
}
