mod camera;
mod color;
mod draw;
mod schedule;
mod sim;

use bevy::prelude::*;

use camera::CameraPlugin;
use color::ColorPlugin;
use draw::DrawPlugin;
use schedule::SchedulePlugin;
use sim::SimPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DrawPlugin)
        .add_plugins(ColorPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(SimPlugin)
        .add_plugins(SchedulePlugin)
        .run();
}
