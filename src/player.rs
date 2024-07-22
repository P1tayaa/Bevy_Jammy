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
	mut player_q: Query<&mut Transform, With<Player>>,
	time: Res<Time>,
	mut camera_q: Query<&Transform, (With<Camera3d>, Without<Player>)>
){
	for mut player_transform in player_q.iter_mut() {
		let cam = match camera_q.get_single() {
			Ok(c) => c,
			Err(e) => Err(format!("could not retrive camera during movement")).unwrap()
		};
		let mut direction: Vec3 = Vec3::ZERO;
		if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
			println!("moving left");
			direction += cam.left().as_vec3();
		}
		if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
			println!("moving right");
			direction += cam.right().as_vec3();
		}
		if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyW]) {
			println!("Jump");
			direction += cam.forward().as_vec3();
		}
		if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
			println!("roll");
			direction += cam.back().as_vec3();
		}

		direction.y = 0.;
		let movement = direction.normalize_or_zero() * 2. * time.delta_seconds();
		player_transform.translation += movement;
	}

}