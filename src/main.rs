use std::any::Any;

use avian3d::prelude::*;
use bevy::prelude::*;

mod player;
use player::{Player, PlayerPlugin};


fn main() {
    App::new()
        // Enable physics
        .add_plugins((DefaultPlugins, PlayerPlugin, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Static physics object with a collision shape
    let player_id = commands.spawn((
        RigidBody::Static,
        Collider::cuboid(15., 0.1, 15.),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(5., 0.1, 15.)),
            material: materials.add(Color::WHITE),
            ..default()
        },
    )).id();

    // Dynamic physics object with a collision shape and initial angular velocity
    commands.spawn((
		Player,
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        },
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
        ..default()
    });
}