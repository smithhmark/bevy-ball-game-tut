use bevy::prelude::*;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;

pub mod systems;

use crate::{events, AppState};
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<events::GameOver>()
            .add_plugins(score::ScorePlugin)
            .add_plugins(star::StarPlugin)
            .add_plugins(enemy::EnemyPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_systems(
                Update,
                systems::toggle_simulation.run_if(in_state(AppState::Game)),
            );
    }
}

#[derive(States, Hash, Default, PartialEq, Eq, Debug, Clone, Copy)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
