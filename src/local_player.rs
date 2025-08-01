
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{common::{death::Killable, physics::gravity::Gravity}, ground::Groundable, networking::networked_players::NetworkedPlayer, player::{dash_controller::{DashController, DASH_COOLDOWN, DASH_DURATION, DASH_KEY, DASH_SPEED}, death::Respawnable, horizontal_movement_controller::{AirbourneHorizontalMovementController, GroundedHorizontalMovementController}, jump_controller::JumpController, look_state::PlayerLookState, physics_controller::PhysicsController, wall_jump_controller::{WallJumpController, WallStickable}}, stage::{stage_builder::stage_creator::TILE_SIZE, stage_objects::StageObject}, wall::Wallable};

pub const FORCE_MUL: f32 = TILE_SIZE / 16.0;

pub const PLAYER_SIZE: Vec2 = Vec2::new(TILE_SIZE * 0.8, TILE_SIZE * 0.8);

const PLAYER_ACCELERATION: f32 = 1000.0 * FORCE_MUL;
const PLAYER_DECELERATION: f32 = 1000.0 * FORCE_MUL;
pub const MAX_HORIZONTAL_SPEED: f32 = 225.0 * FORCE_MUL;

const PLAYER_MIN_VELOCITY: Vec2 = Vec2::new(-500.0 * FORCE_MUL, -400.0 * FORCE_MUL);
const PLAYER_MAX_VELOCITY: Vec2 = Vec2::new(500.0 * FORCE_MUL, 400.0 * FORCE_MUL);

const PLAYER_JUMP_SPEED: f32 = 200.0 * FORCE_MUL;
const PLAYER_JUMP_DURATION: f64 = 0.4;
const PLAYER_COYOTE_TIME: f64 = 0.1;

const PLAYER_WALL_JUMP_IN_FORCE: Vec2 = Vec2::new(150.0 * FORCE_MUL, 200.0 * FORCE_MUL);
const PLAYER_WALL_JUMP_OUT_FORCE: Vec2 = Vec2::new(275.0 * FORCE_MUL, 200.0 * FORCE_MUL);
const PLAYER_WALL_FRICTION_COEFFICIENT: f32 = 0.03 * FORCE_MUL;
const PLAYER_WALL_STICK_DURATION: f64 = 0.3;

pub const PLAYER_MAX_GRAVITY: f32 = 1000.0 * FORCE_MUL;
pub const PLAYER_GRAVITY_ACCELERATION: f32 = 1000.0 * FORCE_MUL;

const PLAYER_RESPAWN_DELAY: f64 = 0.5;

#[derive(Component)]
pub struct LocalPlayer;

#[derive(Bundle)]
pub struct LocalPlayerBundle {
    local_player_marker: LocalPlayer,
    sprite_bundle: SpriteBundle,
    rigid_body: RigidBody,
    ccd: Ccd,
    collider: Collider,
    sensor: Sensor,
    restitution: Restitution,
    friction: Friction,
    velocity: Velocity,
    gravity: Gravity,
    rapier_gravity_scale: GravityScale,
    groundable_marker: Groundable,
    wallable_marker: Wallable,
    colliding_entities: CollidingEntities,
    physics_controller: PhysicsController,
    jump_controller: JumpController,
    wall_jump_controller: WallJumpController,
    wall_stickable: WallStickable,
    grounded_horizontal_movement_controller: GroundedHorizontalMovementController,
    airbourne_horizontal_movement_controller: AirbourneHorizontalMovementController,
    dash_controller: DashController,
    respawnable: Respawnable,
    stage_object: StageObject,
    killable: Killable
}
impl LocalPlayerBundle {
    pub fn new(pos: Vec3, stage_id: usize) -> Self {
        let mut p = LocalPlayerBundle::default();
        p.sprite_bundle.transform.translation = pos;
        p.respawnable.translation = pos;
        return p;
    }
}
impl Default for LocalPlayerBundle {
    fn default() -> Self {
        LocalPlayerBundle {
            local_player_marker: LocalPlayer,
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    scale: PLAYER_SIZE.extend(1.0),
                    ..default()
                },
                ..default()
            },
            rigid_body: RigidBody::Dynamic,
            ccd: Ccd::enabled(),
            collider: Collider::ball(0.5),
            sensor: Sensor,
            restitution: Restitution::coefficient(0.0),
            friction: Friction::coefficient(0.0),
            velocity: Velocity::linear(Vec2::ZERO),
            gravity: Gravity {
                max_force: PLAYER_MAX_GRAVITY,
                current_force: 0.0,
                acceleration: PLAYER_GRAVITY_ACCELERATION,
            },
            rapier_gravity_scale: GravityScale(0.0),
            groundable_marker: Groundable,
            colliding_entities: CollidingEntities::default(),
            physics_controller: PhysicsController {
                max_velocity: PLAYER_MAX_VELOCITY,
                min_velocity: PLAYER_MIN_VELOCITY,
            },
            jump_controller: JumpController {
                key: KeyCode::KeyW,
                force: PLAYER_JUMP_SPEED,
                duration: PLAYER_JUMP_DURATION,
                last_jump_pressed_time: 0.0,
                last_jump_released_time: 0.0,
                last_grounded: 0.0,
                coyote_time: PLAYER_COYOTE_TIME,
            },
            wall_jump_controller: WallJumpController {
                force_in: PLAYER_WALL_JUMP_IN_FORCE,
                force_out: PLAYER_WALL_JUMP_OUT_FORCE,
                friction_coefficient: PLAYER_WALL_FRICTION_COEFFICIENT,
            },
            wall_stickable: WallStickable {
                wall_stick_time: PLAYER_WALL_STICK_DURATION,
            },
            grounded_horizontal_movement_controller: GroundedHorizontalMovementController {
                left_key: KeyCode::KeyA,
                right_key: KeyCode::KeyD,
                acceleration: PLAYER_ACCELERATION,
                deceleration: PLAYER_DECELERATION,
                max_speed: MAX_HORIZONTAL_SPEED,
            },
            airbourne_horizontal_movement_controller: AirbourneHorizontalMovementController {
                left_key: KeyCode::KeyA,
                right_key: KeyCode::KeyD,
                acceleration: PLAYER_ACCELERATION / 1.0,
                deceleration: PLAYER_DECELERATION,
                max_speed: MAX_HORIZONTAL_SPEED,
            },
            respawnable: Respawnable {
                translation: Vec3::default(),
                delay_in_seconds: PLAYER_RESPAWN_DELAY,
            },
            stage_object: StageObject,
            killable: Killable,
            wallable_marker: Wallable,
            dash_controller: DashController::default(),
        }
    }
}

//TODO: Remove this trash below

pub fn load_player_sprite(
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut Sprite), Or<(With<LocalPlayer>, With<NetworkedPlayer>)>>,
    mut commands: Commands
) {
    let tilemap: Handle<Image> = asset_server.load("object_tilemap.png");
    let player_rect = Rect::new(TILE_SIZE * 2.0, TILE_SIZE, TILE_SIZE * 3.0, TILE_SIZE * 2.0);

    for (e, mut s) in &mut query {
        commands.entity(e).try_insert(tilemap.clone());
        s.rect = Some(player_rect);
        s.custom_size = Some(Vec2::new(1.0, 1.0));
    }
}

pub fn update_player_look_direction(
    mut query: Query<(&PlayerLookState, &mut Sprite)>,
) {
    
    for (ls, mut s) in &mut query {
        match ls {
            PlayerLookState::LookingLeft => s.flip_x = true,
            PlayerLookState::LookingRight => s.flip_x = false,
        }
    }
}


