extern crate blizzard_engine;
extern crate blizzard_engine_derive;

// use blizzard_engine::ecs::{ComponentRegistry, EntityManager, World};
// use blizzard_engine::game::Game;
// use blizzard_engine_derive::ComponentRegistry;
// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};

// // World
// #[derive(Debug)]
// struct MyWorld {
//     entity_manager: EntityManager,
//     position: PositionRegistry,
//     counters: CounterRegistry,
//     others: OtherRegistry,
// }

// impl World for MyWorld {
//     fn new() -> Self {
//         Self {
//             entity_manager: EntityManager::new(),
//             position: PositionRegistry::new(),
//             counters: CounterRegistry::new(),
//             others: OtherRegistry::new(),
//         }
//     }
//     fn run_systems(&mut self) {
//         position_system(&mut self.position.components);
//         counter_system(&mut self.counters.components);
//         other_system(&mut self.others.components);
//     }
// }

// #[derive(ComponentRegistry, Debug)]
// struct OtherRegistry {
//     components: HashMap<u32, i32>,
// }

// #[derive(ComponentRegistry, Debug)]
// struct PositionRegistry {
//     components: HashMap<u32, Position>,
// }

// #[derive(Debug, Clone, Copy)]
// struct Position {
//     x: i32,
//     y: i32,
// }

// impl Position {
//     fn new() -> Self {
//         Self { x: 0, y: 0 }
//     }
//     fn displace(&mut self, x: i32, y: i32) {
//         self.x += x;
//         self.y += y;
//     }
// }

// #[derive(ComponentRegistry, Debug)]
// struct CounterRegistry {
//     components: HashMap<u32, i32>,
// }

// // Systems
// fn position_system(positions: &mut HashMap<u32, Position>) {
//     for (_, position) in positions.iter_mut() {
//         position.displace(1, 1);
//     }
// }

// fn counter_system(counters: &mut HashMap<u32, i32>) {
//     for (_, counter) in counters.iter_mut() {
//         *counter *= 2;
//     }
// }

// fn other_system(others: &mut HashMap<u32, i32>) {
//     for (_, other) in others.iter_mut() {
//         *other -= 1;
//     }
// }

// // Games
// struct MyGame {
//     world: MyWorld,
//     counter: u32,
// }

// impl Game<i32, i32> for MyGame {
//     fn world_config(&mut self) {
//         // Create Entities
//         let ent1 = self.world.entity_manager.create_entity();
//         let ent2 = self.world.entity_manager.create_entity();
//         let ent3 = self.world.entity_manager.create_entity();

//         // Add components
//         self.world.position.add(ent1, Position::new());
//         self.world.position.add(ent3, Position::new());

//         self.world.counters.add(ent2, 1);
//         self.world.counters.add(ent3, 1);

//         self.world.others.add(ent1, 0);

//         // Create multiple entities
//         let entities = self.world.entity_manager.create_n_entities(2);

//         // Add components to many entities
//         self.world.counters.add_many(&entities, 1);
//         self.world.position.add_many(&entities, Position::new());
//     }

//     fn update(&mut self, _: i32, _: Arc<Mutex<i32>>) {
//         self.world.run_systems();
//         self.counter += 1;
//         println!("{:#?}", self.world);
//     }

//     fn render(&mut self) {}

//     fn end_game(&self) -> bool {
//         self.counter > 10
//     }
// }

// fn new_game(world: MyWorld) -> impl Game<i32, i32> {
//     MyGame {
//         counter: 0,
//         world: world,
//     }
// }

// fn main() {
//     let game = new_game(MyWorld::new());

//     blizzard_engine::start(game, 0, 0, );
// }

fn main() {
    blizzard_engine::core::windows::main();
}
