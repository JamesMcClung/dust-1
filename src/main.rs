mod camera;
mod color;
mod draw;
mod fps;
mod schedule;
mod sim;
mod zero;

use bevy::prelude::*;

use camera::CameraPlugin;
use color::ColorPlugin;
use draw::DrawPlugin;
use schedule::SchedulePlugin;
use sim::SimPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins)
        .add_plugins(fps::FpsPlugin)
        .add_plugins(DrawPlugin)
        .add_plugins(ColorPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(SimPlugin)
        .add_plugins(SchedulePlugin)
        .run();
}
