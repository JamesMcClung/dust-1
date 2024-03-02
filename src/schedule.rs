use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub enum SimSet {
    Gravity,
    Gas,
    Draw,
    Recolor,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum SimState {
    Playing,
    Paused,
    Stepping,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_state(SimState::Playing)
            .add_systems(Update, (
                handle_state_inputs,
                stop_stepping.run_if(in_state(SimState::Stepping)),
            ))
            .configure_sets(
                Update,
                (SimSet::Gravity, SimSet::Gas, SimSet::Draw, SimSet::Recolor)
                    .chain()
                    .run_if(in_state(SimState::Playing).or_else(in_state(SimState::Stepping)))
            )
        ;
    }
}

fn handle_state_inputs(
    mut next_state: ResMut<NextState<SimState>>,
    state: Res<State<SimState>>,
    inputs: Res<ButtonInput<KeyCode>>,
) {
    if inputs.just_pressed(KeyCode::Space) {
        match state.get() {
            SimState::Paused => next_state.set(SimState::Playing),
            SimState::Playing => next_state.set(SimState::Paused),
            _ => (),
        }
    } else if inputs.just_pressed(KeyCode::Period) {
        match state.get() {
            SimState::Paused => next_state.set(SimState::Stepping),
            _ => ()
        }
    }
}

fn stop_stepping(
    mut next_state: ResMut<NextState<SimState>>,
) {
    next_state.set(SimState::Paused)
}