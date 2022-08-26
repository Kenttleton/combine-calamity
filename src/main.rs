use bevy::prelude::*;
mod ui;

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Splash,
    Menu,
    Game,
    Pause,
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        // Declare the game state, and set its startup value
        .add_state(GameState::Splash)
        // Adds the plugins for each state
        .add_plugin(ui::UIPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
