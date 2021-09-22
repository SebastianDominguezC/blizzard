use crate::core::application::Application;
use crate::game::Game;

pub fn run<T: Game>(game: T) {
    let mut app: Application<T> = Application::create(game);
    app.start();
}
