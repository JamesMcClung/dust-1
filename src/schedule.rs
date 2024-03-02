use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub enum SimSet {
    Gravity,
    Gas,
    Draw,
    Recolor,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (SimSet::Gravity, SimSet::Gas, SimSet::Draw, SimSet::Recolor).chain());
    }
}