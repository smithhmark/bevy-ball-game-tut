use super::SimulationState;
use bevy::prelude::*;

pub fn toggle_simulation(
    kb: Res<Input<KeyCode>>,
    sim_state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    if kb.just_pressed(KeyCode::Space) {
        println!("{sim_state:?}");
        if *sim_state.get() == SimulationState::Running {
            next_state.set(SimulationState::Paused);
        }
        if *sim_state.get() == SimulationState::Paused {
            next_state.set(SimulationState::Running);
        }
    }
}
