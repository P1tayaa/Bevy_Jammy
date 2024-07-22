use avian3d::{math::*, prelude::*};
use bevy::{ecs::query::Has, prelude::*};

static LEFT_LANE: f32 = -2.5;
static MIDDLE_LANE: f32 = 0.;
static RIGHT_LANE: f32 = 2.5; 
static MOVE_SPEED: f32 = 5.;

//-------------------------------------------------------------------------
//------------------AVIAN DYNAMIC BODY EXAMPLE TEMPLATE//------------------
//-------------------------------------------------------------------------

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>().add_systems(
            Update,
            (
                player_movement,
                update_grounded,
                movement,
                apply_movement_damping,
            )
                .chain(),
        );
    }
}

/// An event sent for a movement input action.
#[derive(Event)]
pub enum MovementAction {
    Move(Vector2),
    Jump,
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;
/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(Scalar);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, PI * 0.45)
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(
                caster_shape,
                Vector::ZERO,
                Quaternion::default(),
                Dir3::NEG_Y,
            )
            .with_max_time_of_impact(0.2),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse, max_slope_angle);
        self
    }
}


fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<CharacterController>,
    >,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        Has<Grounded>,
    )>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for event in movement_event_reader.read() {
        for (movement_acceleration, jump_impulse, mut linear_velocity, is_grounded) in
            &mut controllers
        {
            match event {
                MovementAction::Move(direction) => {
                    linear_velocity.x += direction.x * movement_acceleration.0 * delta_time;
                    linear_velocity.z -= direction.y * movement_acceleration.0 * delta_time;
                }
                MovementAction::Jump => {
                    if is_grounded {
                        linear_velocity.y = jump_impulse.0;
                    }
                }
            }
        }
    }
}

/// Slows down movement in the XZ plane.
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}

//-------------------------------------------------------------------------
//-------------------------------------------------------------------------
//-------------------------------------------------------------------------



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
		app.add_systems(Startup, spawn_player);
			// .add_systems(Update, player_movement);
	}
}

fn spawn_player(mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
	commands.spawn((
		Player,
		CharacterControllerBundle::new(Collider::capsule(0.4, 1.0)).with_movement(
            30.0,
            0.92,
            7.0,
            (30.0 as Scalar).to_radians(),
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        GravityScale(2.0),
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
	mut exit: EventWriter<AppExit>,
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
			slide.target = Vec3::new(target_x, player_transform.translation.y, player_transform.translation.z);
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
			slide.target = Vec3::new(target_x, player_transform.translation.y, player_transform.translation.z);
		}
		if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW, KeyCode::Space]) {
			println!("Jump");
		}
		if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
			println!("roll");
		}
		if keys.pressed(KeyCode::Escape) {
			exit.send(AppExit::Success);
		}

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

// fn run_if_player_unlocked(mut player_q: Query<&AnimLock, With<Player>>) -> bool {
// 	if let Ok(anim_lock) = player_q.get_single() {
//         anim_lock.0 // Access the boolean value inside AnimLock
//     } else {
//         false // Or handle the error case as needed
//     }
// }