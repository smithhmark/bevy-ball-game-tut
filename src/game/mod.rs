use bevy::prelude::*;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(score::ScorePlugin)
            .add_plugins(star::StarPlugin)
            .add_plugins(enemy::EnemyPlugin)
            .add_plugins(player::PlayerPlugin);
    }
}
/*
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();

*/
