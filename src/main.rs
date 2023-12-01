use bevy::prelude::*;

mod events;
mod systems;

mod enemy;
mod player;
mod score;
mod star;

use events::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugins(score::ScorePlugin)
        .add_plugins(star::StarPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}
