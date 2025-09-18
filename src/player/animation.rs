
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{common::animation_controller::{AnimationController, AnimationState}, ground::Grounded, player::jump_controller::{Falling, Jumping}};

#[derive(Clone, Copy)]
pub enum PlayerAnimationState {
    Idle,
    Running,
    Jumping,
    Falling,
    Hovering,
    OnWall
}


pub fn update_player_animation_state(
    mut query: Query<(&mut AnimationState, &LinearVelocity, Option<&Grounded>, Option<&Jumping>, Option<&Falling>)>
) {
    for (mut anim_state, linvel, grounded_opt, jumping_opt, falling_opt) in &mut query {
        let mut state = PlayerAnimationState::Idle;

        if grounded_opt.is_some() {
            if linvel.x.abs() >= 30.0 { state = PlayerAnimationState::Running; }
        } else {
            if linvel.y >= 50.0 { state = PlayerAnimationState::Jumping; }
            else if linvel.y <= -50.0 { state = PlayerAnimationState::Falling; }
            else { state = PlayerAnimationState::Hovering; }
        }

        if anim_state.0 != state as u32 {
            anim_state.0 = state as u32;
        }
    }
}