use bevy::prelude::*;
use avian3d::prelude::*;

static LEFT_LANE: f32 = -2.5;
static MIDDLE_LANE: f32 = 0.;
static RIGHT_LANE: f32 = 2.5; 
static MOVE_SPEED: f32 = 5.;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Sliding {
	is_sliding: bool,
	target: Vec3,
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
		Sliding {
			is_sliding: false,
			target: Vec3::ZERO,
		},
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
	mut player_q: Query<(&mut Transform, &mut Sliding), With<Player>>,
	time: Res<Time>,
	camera_q: Query<&Transform, (With<Camera3d>, Without<Player>)>
){
	for (mut player_transform, mut slide) in player_q.iter_mut() {
		let cam = match camera_q.get_single() {
			Ok(c) => c,
			Err(e) => Err(format!("could not retrive camera during movement, err: {}", e)).unwrap()
		};
		let current_x = player_transform.translation.x;
		let mut target_x = 0.;

		if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
			println!("moving left");
			if current_x > MIDDLE_LANE {
				target_x = MIDDLE_LANE;
				slide.is_sliding = true;
			} else if current_x > LEFT_LANE {
				target_x = LEFT_LANE;
				slide.is_sliding = true;
			}
		}
		if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
			println!("moving right");
			if current_x < MIDDLE_LANE {
				target_x = MIDDLE_LANE;
				slide.is_sliding = true;
			} else if current_x < RIGHT_LANE {
				target_x = RIGHT_LANE;
				slide.is_sliding = true;
			}
		}
		if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyW]) {
			println!("Jump");
		}
		if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
			println!("roll");
		}

		slide.target = Vec3::new(target_x, player_transform.translation.y, player_transform.translation.z);
		let mut direction = slide.target - player_transform.translation;
		direction.y = 0.;
		let movement = direction.normalize_or_zero() * MOVE_SPEED * time.delta_seconds();
		if direction.length() < movement.length() {
            player_transform.translation = slide.target;
            slide.is_sliding = false;  // Stop sliding once the target is reached
			println!("stopped sliding");
        } else {
            player_transform.translation += movement;
        }
	}
}