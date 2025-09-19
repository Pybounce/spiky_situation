
use bevy::prelude::*;

use crate::{ground::Grounded, player::look_state::PlayerLookState, stage::stage_builder::stage_creator::TILE_SIZE, wall::TouchingWall};

pub const FORCE_MUL: f32 = TILE_SIZE / 16.0;

pub const PLAYER_SIZE: Vec2 = Vec2::new(TILE_SIZE, TILE_SIZE);

pub const PLAYER_ACCELERATION: f32 = 1000.0 * FORCE_MUL;
pub const PLAYER_DECELERATION: f32 = 1000.0 * FORCE_MUL;
pub const MAX_HORIZONTAL_SPEED: f32 = 225.0 * FORCE_MUL;

pub const PLAYER_MIN_VELOCITY: Vec2 = Vec2::new(-500.0 * FORCE_MUL, -400.0 * FORCE_MUL);
pub const PLAYER_MAX_VELOCITY: Vec2 = Vec2::new(500.0 * FORCE_MUL, 400.0 * FORCE_MUL);

pub const PLAYER_JUMP_SPEED: f32 = 200.0 * FORCE_MUL;
pub const PLAYER_JUMP_DURATION: f64 = 0.4;
pub const PLAYER_COYOTE_TIME: f64 = 0.1;

pub const PLAYER_WALL_JUMP_IN_FORCE: Vec2 = Vec2::new(150.0 * FORCE_MUL, 200.0 * FORCE_MUL);
pub const PLAYER_WALL_JUMP_OUT_FORCE: Vec2 = Vec2::new(275.0 * FORCE_MUL, 200.0 * FORCE_MUL);
pub const PLAYER_WALL_FRICTION_COEFFICIENT: f32 = 0.03 * FORCE_MUL;
pub const PLAYER_WALL_STICK_DURATION: f64 = 0.3;

pub const PLAYER_MAX_GRAVITY: f32 = 1000.0 * FORCE_MUL;
pub const PLAYER_GRAVITY_ACCELERATION: f32 = 1000.0 * FORCE_MUL;

pub const PLAYER_RESPAWN_DELAY: f64 = 0.5;

#[derive(Component)]
pub struct LocalPlayer;


pub fn update_player_look_direction(
    mut query: Query<(&PlayerLookState, &mut Sprite, Option<&TouchingWall>, Option<&Grounded>)>,
) {
    
    for (ls, mut s, wall_opt, grounded_opt) in &mut query {
        match ls {
            PlayerLookState::LookingLeft => s.flip_x = true,
            PlayerLookState::LookingRight => s.flip_x = false,
        }
        if grounded_opt.is_none() {
            if let Some(wall) = wall_opt {
                match wall {
                    TouchingWall::Left => s.flip_x = true,
                    TouchingWall::Right => s.flip_x = false,
                }
            }
        }

    }
}


