use bevy::prelude::*;
use avian3d::prelude::*;
use core::any::*;

static LEFT_LANE: f32 = 5.;
static MIDDLE_LANE: f32 = 10.;
static RIGHT_LANE: f32 = 15.; 

#[derive(Component, Debug)]
pub struct Player;


// components
#[derive(Component, Debug)]
pub struct Speed {
	value: i32,
}
#[derive(Component, Debug)]
pub struct Id {
	id: TypeId,
}


pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, spawn_player)
			.add_systems(Update, player_movement);
	}
}

fn spawn_player(mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
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
}

fn player_movement(keys: Res<ButtonInput<KeyCode>>,
	mut player_query: Query<&mut Transform, With<Player>>
){
	let position = player_query.get_single_mut().unwrap();
	let mut direction: Vec3;
	if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
		println!("moving left");
	}
	if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
		println!("moving right");
	}
	if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyW]) {
		println!("Jump");
	}
	if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
		println!("roll");
	}

	// position.transform = 
}