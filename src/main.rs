use avian3d::prelude::*;
use bevy::prelude::*;

mod player;
mod camera;
mod world;

use player::PlayerPlugin;
use camera::CameraPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldPlugin, CameraPlugin, PlayerPlugin, PhysicsPlugins::default()))
        .run();
}
