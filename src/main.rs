use bevy::prelude::*;

mod events;
mod systems;

mod game;
mod main_menu;

use events::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(game::GamePlugin)
        .add_plugins(main_menu::MainMenuPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}
