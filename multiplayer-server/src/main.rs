#[macro_use]
extern crate serde_derive;
extern crate blizzard_engine;
extern crate blizzard_engine_derive;
extern crate serde;
extern crate serde_json;

use blizzard_engine::ecs::{ComponentRegistry, EntityManager, World};
use blizzard_engine::game::Game;
use blizzard_engine_derive::ComponentRegistry;
use blizzard_server::server::Server;

use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

// World definition
#[derive(Debug, Clone)]
struct MyWorld {
    entity_manager: EntityManager,
    position: PositionRegistry,
}

impl World<Input> for MyWorld {
    fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            position: PositionRegistry::new(),
        }
    }
    fn run_systems(&mut self, input: Input) {
        position_system(&mut self.position.components, input);
    }
}

// Components
#[derive(ComponentRegistry, Debug, Clone)]
struct PositionRegistry {
    components: HashMap<u32, Position>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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

// Systems
fn position_system(positions: &mut HashMap<u32, Position>, input: Input) {
    for (_, position) in positions.iter_mut() {
        position.displace(input.x, input.y);
    }
}

// Game
#[derive(Clone)]
struct MyGame {
    world: MyWorld,
    counter: i32,
}

impl Game<SharedState, Input> for MyGame {
    fn world_config(&mut self) {
        // Create multiple entities
        let entities = self.world.entity_manager.create_n_entities(2);

        // Add components to many entities
        self.world.position.add_many(&entities, Position::new());
    }

    fn update(&mut self, input: Input, shared_state: Arc<Mutex<SharedState>>) {
        self.world.run_systems(input);
        self.counter += 1;
        shared_state.lock().unwrap().counter = input.x;
        shared_state.lock().unwrap().registry = self
            .world
            .position
            .components
            .iter()
            .map(|(_, lol)| *lol)
            .collect();
    }

    fn render(&mut self) {}

    fn end_game(&self) -> bool {
        false
    }
}

// Game creator
fn new_game(world: MyWorld) -> MyGame {
    MyGame {
        counter: 0,
        world: world,
    }
}

// Shared state definition
#[derive(Serialize, Deserialize, Debug, Clone)]
struct SharedState {
    counter: i32,
    registry: Vec<Position>,
}

impl SharedState {
    pub fn new(counter: i32) -> Self {
        Self {
            counter,
            registry: vec![],
        }
    }
}

// Input definition
#[derive(Debug, Clone, Copy)]
struct Input {
    x: i32,
    y: i32,
}
impl Input {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    fn displace(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

// Message definition
#[derive(Serialize, Deserialize)]
enum Message {
    None,
    W,
    A,
    S,
    D,
}

fn handle_client_message(receiver: Receiver<Message>, input: Arc<Mutex<Input>>) -> Input {
    for message in receiver {
        match message {
            Message::None => {
                input.lock().unwrap().displace(0, 0);
            }
            Message::W => {
                input.lock().unwrap().displace(0, 1);
            }
            Message::A => {
                input.lock().unwrap().displace(-1, 0);
            }
            Message::S => {
                input.lock().unwrap().displace(0, -1);
            }
            Message::D => {
                input.lock().unwrap().displace(1, 0);
            }
        }
    }
    Input::new()
}

// Main function
fn main() {
    let port = 8888;
    let max_games = 4;
    let max_players = 2;
    let world = MyWorld::new();

    let shared_state = SharedState::new(0);

    let game = new_game(world);
    // The data that a client message will manipulate
    let input_type = Input::new();
    let hanlde_input = &handle_client_message;

    Server::new(
        port,
        max_games,
        max_players,
        game,
        shared_state,
        input_type,
        hanlde_input,
    );
}
