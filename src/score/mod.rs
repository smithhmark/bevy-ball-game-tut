use bevy::prelude::*;

pub mod resources;
pub mod systems;

use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Update, update_score)
            .add_systems(Update, update_high_scores)
            .add_systems(Update, high_scores_updated)
            .init_resource::<HighScores>();
    }
}
