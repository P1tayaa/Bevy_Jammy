use bevy::prelude::*;
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
		app.add_systems(Update, player_movement);
	}
}

pub fn player_movement(keys: Res<ButtonInput<KeyCode>>,
	mut query: Query<&mut Transform, With<Player>>
){
	let position = query.get_single_mut().unwrap();
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
}