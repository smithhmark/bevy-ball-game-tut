use bevy::prelude::*;

mod events;
mod systems;

mod game;
mod main_menu;

use events::*;
use systems::*;

#[derive(States, Hash, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugins(game::GamePlugin)
        .add_plugins(main_menu::MainMenuPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_main_menu_state)
        .run();
}
