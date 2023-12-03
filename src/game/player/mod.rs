use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

use crate::AppState;

use super::SimulationState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(OnExit(AppState::Game), despawn_player)
            .add_systems(
                Update,
                (
                    player_movement.before(confine_player_movement),
                    confine_player_movement,
                    enemy_hit_player,
                    player_hit_star,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
        //.add_systems(Update, enemy_hit_player)
        //.add_systems(Update, player_hit_star);
    }
}
