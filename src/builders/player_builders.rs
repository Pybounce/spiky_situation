use bevy::{ecs::system::EntityCommands, platform::collections::HashMap, prelude::*};
use avian2d::prelude::*;

use crate::{common::{animated_sprite::SpriteAnimator, animation_controller::{AnimationController, AnimationState}, death::{DelayedDeathMarker, Killable}, physics::{avian_ex::ManyCollidingEntities, gravity::Gravity, layers::GamePhysicsLayer}, splat::SplatOnDeath}, ground::Groundable, local_player::{LocalPlayer, PLAYER_MAX_GRAVITY, PLAYER_SIZE}, player::{animation::PlayerAnimationState, dash_controller::DashController, death::Respawnable, horizontal_movement_controller::{AirbourneHorizontalMovementController, GroundedHorizontalMovementController}, jump_controller::JumpController, physics_controller::PhysicsController, wall_jump_controller::{WallJumpController, WallStickable}}, stage::{stage_builder::stage_creator::TILE_SIZE, stage_objects::StageObject}, wall::Wallable};
use crate::local_player::*;

#[derive(Resource)]
pub struct PlayerBuilder {
    player_atlas: Handle<Image>
}

impl PlayerBuilder {
    pub fn build_player_corpse(&self, entity_commands: &mut EntityCommands, pos: Vec3) {
        let player_corpse_rect = Rect::new(TILE_SIZE * 1.0, TILE_SIZE, TILE_SIZE * 2.0, TILE_SIZE * 2.0);

        entity_commands.try_insert((
            Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                rect: Some(player_corpse_rect),
                image: self.player_atlas.clone(),
                ..default()
            },
            Transform::from_scale(PLAYER_SIZE.extend(1.0)).with_translation(pos),
            DelayedDeathMarker::from_secs(5.0),
            RigidBody::Dynamic,
            LinearVelocity(Vec2::new(0.0, 200.0)),
            Gravity {
                max_force: 400.0,
                current_force: 0.0,
                acceleration: 3000.0,
            }
        ));
    }
    pub fn build_player(entity_commands: &mut EntityCommands, asset_server: &AssetServer, spawn_pos: Vec3) {
        let atlas: Handle<Image> = asset_server.load("player.png");
        let run_rects = vec![
            Rect::new(0.0, 0.0, TILE_SIZE, TILE_SIZE),
            Rect::new(TILE_SIZE, 0.0, TILE_SIZE * 2.0, TILE_SIZE),
            Rect::new(TILE_SIZE * 2.0, 0.0, TILE_SIZE * 3.0, TILE_SIZE),
            Rect::new(TILE_SIZE * 3.0, 0.0, TILE_SIZE * 4.0, TILE_SIZE),
        ];

        let dance_rects = vec![
            Rect::new(0.0, TILE_SIZE * 2.0, TILE_SIZE, TILE_SIZE * 3.0),
            Rect::new(TILE_SIZE, TILE_SIZE * 2.0, TILE_SIZE * 2.0, TILE_SIZE * 3.0),
            Rect::new(TILE_SIZE * 2.0, TILE_SIZE * 2.0, TILE_SIZE * 3.0, TILE_SIZE * 3.0),
            Rect::new(TILE_SIZE * 3.0, TILE_SIZE * 2.0, TILE_SIZE * 4.0, TILE_SIZE * 3.0),
        ];

        let jumping_rects = vec![
            Rect::new(0.0, TILE_SIZE, TILE_SIZE, TILE_SIZE * 2.0),
        ];
        
        let falling_rects = vec![
            Rect::new(TILE_SIZE, TILE_SIZE, TILE_SIZE * 2.0, TILE_SIZE * 2.0),
        ];

        let hovering_rects = vec![
            Rect::new(TILE_SIZE * 2.0, TILE_SIZE, TILE_SIZE * 3.0, TILE_SIZE * 2.0),
        ];

        let idle_rects = vec![
            Rect::new(TILE_SIZE * 0.0, TILE_SIZE * 3.0, TILE_SIZE * 1.0, TILE_SIZE * 4.0),
            Rect::new(TILE_SIZE * 1.0, TILE_SIZE * 3.0, TILE_SIZE * 2.0, TILE_SIZE * 4.0),
            Rect::new(TILE_SIZE * 2.0, TILE_SIZE * 3.0, TILE_SIZE * 3.0, TILE_SIZE * 4.0),
            Rect::new(TILE_SIZE * 3.0, TILE_SIZE * 3.0, TILE_SIZE * 4.0, TILE_SIZE * 4.0),
            Rect::new(TILE_SIZE * 4.0, TILE_SIZE * 3.0, TILE_SIZE * 5.0, TILE_SIZE * 4.0),
        ];

        let wall_grab_rects = vec![
            Rect::new(0.0, TILE_SIZE * 4.0, TILE_SIZE, TILE_SIZE * 5.0),
        ];

        let mut state_animations = HashMap::<AnimationState, Vec<Rect>>::new();
        state_animations.insert(AnimationState(PlayerAnimationState::Running as u32), run_rects.clone());
        state_animations.insert(AnimationState(PlayerAnimationState::Idle as u32), idle_rects.clone());
        state_animations.insert(AnimationState(PlayerAnimationState::Jumping as u32), jumping_rects.clone());
        state_animations.insert(AnimationState(PlayerAnimationState::Falling as u32), falling_rects.clone());
        state_animations.insert(AnimationState(PlayerAnimationState::Hovering as u32), hovering_rects.clone());
        state_animations.insert(AnimationState(PlayerAnimationState::OnWall as u32), wall_grab_rects.clone());
        state_animations.insert(AnimationState(PlayerAnimationState::Dancing as u32), dance_rects.clone());

        entity_commands.try_insert(((
            LocalPlayer,
            Sprite {
                image: atlas,
                rect: run_rects[0].into(),
                custom_size: Vec2::splat(1.0).into(),
                ..default()
            },
            (SpriteAnimator::new(100, idle_rects),
            AnimationController::new(state_animations),
            AnimationState(PlayerAnimationState::Idle as u32)),
            Transform::from_scale(PLAYER_SIZE.extend(1.0)).with_translation(spawn_pos),
            (RigidBody::Dynamic,
            SweptCcd::default(),
            Collider::circle(0.3),
            LinearVelocity(Vec2::ZERO),
            TransformInterpolation,
            Gravity {
                max_force: PLAYER_MAX_GRAVITY,
                current_force: 0.0,
                acceleration: PLAYER_GRAVITY_ACCELERATION,
            },
            GravityScale(0.0)),
            Groundable,
            //CollidingEntities::default(),
            ManyCollidingEntities::default(),
            PhysicsController {
                max_velocity: PLAYER_MAX_VELOCITY,
                min_velocity: PLAYER_MIN_VELOCITY,
            },
            JumpController {
                key: KeyCode::KeyW,
                force: PLAYER_JUMP_SPEED,
                duration: PLAYER_JUMP_DURATION,
                last_jump_pressed_time: 0.0,
                last_jump_released_time: 0.0,
                last_grounded: 0.0,
                coyote_time: PLAYER_COYOTE_TIME,
            },
            WallJumpController {
                force_in: PLAYER_WALL_JUMP_IN_FORCE,
                force_out: PLAYER_WALL_JUMP_OUT_FORCE,
                friction_coefficient: PLAYER_WALL_FRICTION_COEFFICIENT,
            }),(
            WallStickable {
                wall_stick_time: PLAYER_WALL_STICK_DURATION,
            },
            GroundedHorizontalMovementController {
                left_key: KeyCode::KeyA,
                right_key: KeyCode::KeyD,
                acceleration: PLAYER_ACCELERATION,
                deceleration: PLAYER_DECELERATION,
                max_speed: MAX_HORIZONTAL_SPEED,
            },
            AirbourneHorizontalMovementController {
                left_key: KeyCode::KeyA,
                right_key: KeyCode::KeyD,
                acceleration: PLAYER_ACCELERATION / 1.0,
                deceleration: PLAYER_DECELERATION,
                max_speed: MAX_HORIZONTAL_SPEED,
            },
            Respawnable {
                translation: spawn_pos,
                delay_in_seconds: PLAYER_RESPAWN_DELAY,
            },
            StageObject::Volatile,
            Killable,
            Wallable,
            DashController::default(),
            LockedAxes::ROTATION_LOCKED,
            Sensor,
            SplatOnDeath,
            CollisionLayers::new(GamePhysicsLayer::Player, LayerMask::ALL),
        )));

    }
}


pub fn init_player_builder(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

) {
    let tilemap: Handle<Image> = asset_server.load("object_tilemap.png");
    commands.insert_resource(PlayerBuilder {
        player_atlas: tilemap
    });
}