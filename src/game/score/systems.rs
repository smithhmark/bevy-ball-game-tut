use bevy::prelude::*;

use super::resources::*;
use crate::events::*;

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.read() {
        println!("Sending your final score: {}", event.score);
        high_scores
            .scores
            .push(("Player One".to_string(), event.score));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High scores: {:?}", high_scores.scores);
    }
}
