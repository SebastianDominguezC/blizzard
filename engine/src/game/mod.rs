use crate::ecs::{Component, World};

// Example
struct SomeComponent {
    count: i32,
}

impl SomeComponent {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl Component for SomeComponent {
    fn update(&mut self, input: u32) {
        self.count += input as i32;
    }
    fn print(&self) -> String {
        format!("count: {}", self.count)
    }
    fn component_name(&self) -> String {
        format!("SomeComponent")
    }
}

struct OtherComponent {
    count: i32,
}

impl Component for OtherComponent {
    fn update(&mut self, _: u32) {
        self.count += 1;
    }
    fn print(&self) -> String {
        format!("count: {}", self.count)
    }
    fn component_name(&self) -> String {
        format!("OtherComponent")
    }
}

pub struct Game {
    world: World,
    counter: u32,
}

impl Game {
    pub fn initialize() -> Game {
        Game {
            counter: 0,
            world: World::new(),
        }
    }

    pub fn world_config(&mut self) {
        let ent1 = self.world.add_entity();
        let ent2 = self.world.add_entity();
        let component = Box::new(SomeComponent::new());
        let component2 = Box::new(SomeComponent::new());
        let component3 = Box::new(OtherComponent { count: 2 });
        let mut components: Vec<Box<dyn Component>> = Vec::new();
        components.push(component);
        components.push(component3);
        self.world.add_components(ent1, components);
        self.world.add_component(ent2, component2);
    }

    pub fn update(&mut self, input: u32) {
        self.world.update(input);
        self.counter += 1;
        println!("{:#?}", self.world);
    }

    pub fn render(&mut self) {}

    pub fn end_game(&self) -> bool {
        self.counter > 10
    }
}
