use avian3d::prelude::*;
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
         app.add_systems(Startup, (spawn_floor, spawn_light));
    }
}

fn spawn_floor(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    // Static physics object with a collision shape
    commands.spawn((
    RigidBody::Static,
    Collider::cuboid(15., 0.1, 15.),
    PbrBundle {
        mesh: meshes.add(Cuboid::new(5., 0.1, 15.)),
        material: materials.add(Color::WHITE),
        ..default()
    },
    ));
}

fn spawn_light(mut commands: Commands) {
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0., 8.0, 4.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}