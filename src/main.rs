mod camera;
mod color;
mod draw;
mod fps;
mod schedule;
mod sim;
mod zero;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins)
        .add_plugins(fps::FpsPlugin)
        .add_plugins(draw::DrawPlugin)
        .add_plugins(color::ColorPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(sim::SimPlugin)
        .add_plugins(schedule::SchedulePlugin)
        .run();
}
