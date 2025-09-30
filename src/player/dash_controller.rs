
use std::time::Duration;

use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{common::{player_input::{PlayerInput, PlayerInputController}, physics::gravity::Gravity}, local_player::{FORCE_MUL, MAX_HORIZONTAL_SPEED}, wall::TouchingWall};

use super::look_state::PlayerLookState;

pub const DASH_COOLDOWN: f32 = 1.0;
pub const DASH_SPEED: f32 = 400.0 * FORCE_MUL;
pub const DASH_DURATION: f32 = 0.2;
pub const DASH_END_SPEED: f32 = MAX_HORIZONTAL_SPEED * FORCE_MUL;

#[derive(Component)]
pub struct DashController {
    pub dash_speed: f32,
    pub cooldown_timer: Timer,
    pub duration_timer: Timer,
    pub dash_direction_sign: f32,
    pub dash_end_speed: f32
}

impl Default for DashController {
    fn default() -> Self {
        let mut duration_timer = Timer::from_seconds(DASH_DURATION, TimerMode::Once);
        duration_timer.tick(Duration::from_secs(234));
        Self { 
            dash_speed: DASH_SPEED,
            cooldown_timer: Timer::from_seconds(DASH_COOLDOWN, TimerMode::Once),
            duration_timer: duration_timer,
            dash_direction_sign: 1.0, 
            dash_end_speed: DASH_END_SPEED
        }
    }
}

pub fn start_dashing(
    mut query: Query<(&mut DashController, &PlayerLookState, &PlayerInputController)>,
    time: Res<Time>,

) {
    for (mut dash_controller, player_look_state, input) in &mut query {
        dash_controller.cooldown_timer.tick(time.delta());
        if input.pressed(PlayerInput::Dash) && dash_controller.cooldown_timer.finished() {
            dash_controller.cooldown_timer.reset();
            dash_controller.duration_timer.reset();
            dash_controller.dash_direction_sign = match player_look_state {
                PlayerLookState::LookingLeft => -1.0,
                PlayerLookState::LookingRight => 1.0,
            };
        }
    }
}

pub fn apply_dashing(
    mut query: Query<(&mut DashController, &mut LinearVelocity, &mut Gravity, Option<&TouchingWall>)>,
    time: Res<Time>
) {
    for (mut dash_controller, mut velocity, mut gravity, touching_wall_opt) in &mut query {
        dash_controller.duration_timer.tick(time.delta());

        let mut force_finished = false;
        if let Some(touching_wall) = touching_wall_opt {
            if (dash_controller.dash_direction_sign > 0.0 && *touching_wall == TouchingWall::Left) || (dash_controller.dash_direction_sign < 0.0 && *touching_wall == TouchingWall::Right) {
                let duration = dash_controller.duration_timer.duration();
                dash_controller.duration_timer.set_elapsed(duration);
                force_finished = true;
            }
        }

        if !dash_controller.duration_timer.finished() {
            velocity.0 = Vec2::new(dash_controller.dash_direction_sign * dash_controller.dash_speed, 0.0);
        }
        else if dash_controller.duration_timer.just_finished() && !force_finished {
            velocity.0 = Vec2::new(dash_controller.dash_direction_sign * dash_controller.dash_end_speed, 0.0);
            //gravity.current_force = 0.0;
        }
    }
}