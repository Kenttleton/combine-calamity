use bevy::prelude::*;

mod main_menu;

use main_menu::MainMenuPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
// Use this MainCamera as the camera through out the code. Bevy will complain if multiple cameras are used.
#[derive(Component)]
pub struct MainCamera;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    MainMenu,
    InGame,
}
pub fn main() {
    App::new()
        .add_state(GameState::MainMenu)
        .insert_resource(ClearColor(CLEAR))
        .add_plugin(MainMenuPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
