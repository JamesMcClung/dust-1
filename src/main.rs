mod camera;
mod sim;

use bevy::prelude::*;

use camera::CameraPlugin;
use sim::SimPlugin;

fn main() {
    App::new() //
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(SimPlugin)
        .run();
}
