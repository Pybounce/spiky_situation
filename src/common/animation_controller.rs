
use bevy::{platform::collections::HashMap, prelude::*};

use crate::common::animated_sprite::SpriteAnimator;


#[derive(Component)]
pub struct AnimationController {
    //pub animation_state: AnimationState,
    pub state_animations: HashMap<AnimationState, Vec<Rect>>
}

impl AnimationController {
    pub fn new(state_animations: HashMap<AnimationState, Vec<Rect>>) -> Self {
        return Self {
            state_animations
        };
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Component, Hash)]
pub struct AnimationState(pub u32);



pub fn update_animation_states(
    mut query: Query<(&AnimationController, &mut SpriteAnimator, &AnimationState), Changed<AnimationState>>
) {
    for (controller, mut animator, state) in &mut query {
        if let Some(rects) = controller.state_animations.get(state) {
            animator.set_new_rects(rects.clone());
        }
    }
}