extern crate blizzard_engine;
extern crate blizzard_engine_derive;
extern crate example;

use blizzard_engine::ecs::{ComponentRegistry, EntityManager, World};
use blizzard_engine::game::Game;
use blizzard_engine_derive::ComponentRegistry;
use blizzard_server::server::Server;

use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

// Message, position, and shared state definition
// These are both used by server and client
use example::{Message, Position, SharedState};

// World definition
#[derive(Debug, Clone)]
struct MyWorld {
    entity_manager: EntityManager,
    positions: PositionRegistry,
    counters: CounterRegistry,
    players: PlayerRegistry,
    player_id_map: PlayerIdMap,
}

// Extend world - ECS enabled
impl World<Input> for MyWorld {
    fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            positions: PositionRegistry::new(),
            counters: CounterRegistry::new(),
            players: PlayerRegistry::new(),
            player_id_map: PlayerIdMap::new(),
        }
    }
    fn run_systems(&mut self, input: Input) {
        // Systems to run conditionally
        match input.0 {
            Message::AddPlayer => add_player_system(self, input.1),
            Message::W => {
                update_player_pos_system(self, input.1, Position::displacement(0, 1));
            }
            Message::A => {
                update_player_pos_system(self, input.1, Position::displacement(-1, 0));
            }
            Message::S => {
                update_player_pos_system(self, input.1, Position::displacement(0, -1));
            }
            Message::D => {
                update_player_pos_system(self, input.1, Position::displacement(1, 0));
            }
            Message::RemovePlayer => {
                remove_player_system(self, input.1);
            }
            _ => {}
        }
        // Systems to always run
        counter_system(&mut self.counters.components);
    }
}

// Components
#[derive(ComponentRegistry, Debug, Clone)]
struct CounterRegistry {
    components: HashMap<u32, u32>,
}

#[derive(ComponentRegistry, Debug, Clone)]
struct PositionRegistry {
    components: HashMap<u32, Position>,
}

#[derive(ComponentRegistry, Debug, Clone)]
struct PlayerRegistry {
    components: HashMap<u32, usize>,
}

// Helper state for player id tracking - server tracks different ids than entities
#[derive(Debug, Clone)]
struct PlayerIdMap {
    players: HashMap<usize, u32>,
}
impl PlayerIdMap {
    fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }
}

// Systems
fn add_player_system(world: &mut MyWorld, player_id: usize) {
    let ent = world.entity_manager.create_entity();
    world.players.add(ent, player_id);
    world.positions.add(ent, Position::new());
    world.player_id_map.players.insert(player_id, ent);
}

fn update_player_pos_system(world: &mut MyWorld, player_id: usize, displacement: Position) {
    if let Some(ent) = world.player_id_map.players.get(&player_id) {
        *world
            .positions
            .components
            .entry(*ent)
            .or_insert(displacement) += displacement;
    }
}

fn remove_player_system(world: &mut MyWorld, player_id: usize) {
    if let Some(ent) = world.player_id_map.players.get(&player_id) {
        world.players.components.remove(ent);
        world.positions.components.remove(ent);
        world.entity_manager.remove_entity(*ent);
        world.player_id_map.players.remove(&player_id);
    }
}

fn counter_system(counters: &mut HashMap<u32, u32>) {
    for (_, c) in counters.iter_mut() {
        *c += 1;
    }
}

// Game
#[derive(Clone)]
struct MyGame {
    world: MyWorld,
    counter: i32,
}

// Impl Game - enable app manipulation
impl Game<SharedState, Input> for MyGame {
    fn world_config(&mut self) {
        // Create counter entity
        let entities = self.world.entity_manager.create_n_entities(1);

        // Add components to many entities
        self.world.counters.add_many(&entities, 0);
    }

    fn update(&mut self, input: Input, shared_state: Arc<Mutex<SharedState>>) {
        // Update states
        self.world.run_systems(input);
        self.counter += 1;

        // Update shared state: for client reception
        shared_state.lock().unwrap().counters = self
            .world
            .counters
            .components
            .iter()
            .map(|(_, counter)| *counter)
            .collect();
        shared_state.lock().unwrap().registry = self
            .world
            .positions
            .components
            .iter()
            .map(|(_, positions)| *positions)
            .collect();
    }

    fn reset_input(&mut self, input: Arc<Mutex<Input>>) {
        *input.lock().unwrap() = Input::default();
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

// Input definition
#[derive(Debug, Clone, Copy)]
struct Input(Message, usize);

impl Input {
    fn default() -> Self {
        Self(Message::None, 0)
    }
    fn from(m: Message, id: usize) -> Self {
        Self(m, id)
    }
}

// Handle client messages
fn handle_client_message(receiver: Receiver<(Message, usize)>, input: Arc<Mutex<Input>>) -> Input {
    for (message, id) in receiver {
        println!("Player {} called {:?}", id, message);
        *input.lock().unwrap() = Input::from(message, id);
    }
    Input::default()
}

// Main function
fn main() {
    let port = 8888;
    let max_games = 4;
    let max_players = 2;
    let world = MyWorld::new();
    let shared_state = SharedState::new();
    let game = new_game(world);

    // The data that a client message will manipulate
    let input_type = Input::default();
    let hanlde_input = &handle_client_message;

    // Engine speeds
    let send_data_from_server_rate = 1;
    let server_game_update_rate = 2; // 2 times per second

    // Start server + games
    Server::new(
        port,
        max_games,
        max_players,
        game,
        shared_state,
        input_type,
        hanlde_input,
        send_data_from_server_rate,
        server_game_update_rate,
    );
}
