use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    player_movement.before(confine_player_movement),
                    confine_player_movement,
                ),
            )
            .add_systems(Update, enemy_hit_player)
            .add_systems(Update, player_hit_star);
    }
}
