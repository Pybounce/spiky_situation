
use bevy::prelude::*;

pub enum PlayerAnimationState {
    Idle,
    Running,
    Jumping,
    Falling,
    OnWall
}