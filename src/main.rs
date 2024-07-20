use std::borrow::Borrow;

use bevy::prelude::*;

use std::f32::consts::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // .add_systems(Startup, changeLightMaterial)
        .run();
}

// fn changeLightMaterial(

//     mut object_with_light_material : Query<(SceneBundle)>
// ){

// }

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

    let scene = commands.spawn(SceneBundle {
        scene: asset_server.load("blender/main.glb#Scene0"),
        transform : Transform { scale : Vec3 { x: 10., y: 10., z: 10. },..default()},
        ..default()
    });
    // let x = 0.;
    // let z = 0.;
    // commands
    // .spawn(SpotLightBundle {
    //     transform: Transform::from_xyz(1.0 + x, 2.0, z)
    //         .looking_at(Vec3::new(1.0 + x, 0.0, z), Vec3::X),
    //     spot_light: SpotLight {
    //         intensity: 40_000.0, // lumens
    //         color: Color::WHITE,
    //         shadows_enabled: true,
    //         inner_angle: PI / 4.0 * 0.85,
    //         outer_angle: PI / 4.0,
    //         ..default()
    //     },
    //     ..default()
    // });

    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

