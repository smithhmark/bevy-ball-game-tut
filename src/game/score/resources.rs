use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Score { value: 0 }
    }
}

#[derive(Resource)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> Self {
        Self { scores: Vec::new() }
    }
}
