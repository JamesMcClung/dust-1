use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub enum SimSet {
    Physics,
    Draw,
    Recolor,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (SimSet::Physics, SimSet::Draw, SimSet::Recolor).chain());
    }
}