
use bevy::{platform::collections::HashSet, prelude::*};

pub mod gamepad;
pub mod keyboard;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum PlayerInput {
    Jump,
    Right,
    Left,
    Dash
}

#[derive(Component, Default)]
pub struct PlayerInputController {
    pressed: HashSet<PlayerInput>,
    previous_pressed: HashSet<PlayerInput>,
    just_pressed: HashSet<PlayerInput>,
    just_released: HashSet<PlayerInput>,
}

impl PlayerInputController {
    pub fn press(&mut self, input: PlayerInput) {
        if !self.previous_pressed.contains(&input) {
            self.just_pressed.insert(input);
        }
        self.pressed.insert(input);
        self.just_released.remove(&input);
    }

    pub fn pressed(&self, input: PlayerInput) -> bool {
        return self.pressed.contains(&input);
    }

    pub fn just_pressed(&self, input: PlayerInput) -> bool {
        return self.just_pressed.contains(&input);
    }

    pub fn just_released(&self, input: PlayerInput) -> bool {
        return self.just_released.contains(&input);
    }
}


pub fn reset_player_inputs(
    mut query: Query<&mut PlayerInputController>,
) {
    for mut input in &mut query {
        input.previous_pressed = input.pressed.clone();
        input.just_released = input.pressed.clone();
        input.pressed.clear();
        input.just_pressed.clear();
    }
}
