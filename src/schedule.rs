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
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_state(SimState::Playing)
            .add_systems(Update, toggle_pause)
            .configure_sets(
                Update,
                (SimSet::Gravity, SimSet::Gas, SimSet::Draw, SimSet::Recolor)
                    .chain()
                    .run_if(in_state(SimState::Playing))
            )
        ;
    }
}

fn toggle_pause(
    mut next_state: ResMut<NextState<SimState>>,
    state: Res<State<SimState>>,
    inputs: Res<ButtonInput<KeyCode>>,
) {
    if inputs.just_pressed(KeyCode::Space) {
        match state.get() {
            SimState::Paused => next_state.set(SimState::Playing),
            SimState::Playing => next_state.set(SimState::Paused),
        }
    }
}