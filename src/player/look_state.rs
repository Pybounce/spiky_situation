
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{common::player_input::{PlayerInput, PlayerInputController}, ground::Grounded};

use super::horizontal_movement_controller::{AirbourneHorizontalMovementController, GroundedHorizontalMovementController};

#[derive(Component, Serialize, Deserialize, Clone, Copy)]
pub enum PlayerLookState {
    LookingLeft,
    LookingRight
}



pub fn update_player_airborn_look_state(
    query: Query<(Entity, &AirbourneHorizontalMovementController, &PlayerInputController), Without<Grounded>>,
    mut commands: Commands
) {
    for (e, con, input) in &query {
        if input.pressed(PlayerInput::Right) {
            commands.entity(e).try_insert(PlayerLookState::LookingRight);
        }
        if input.pressed(PlayerInput::Left) {
            commands.entity(e).try_insert(PlayerLookState::LookingLeft);
        }

    }
}

pub fn update_player_grounded_look_state(
    query: Query<(Entity, &GroundedHorizontalMovementController, &PlayerInputController), With<Grounded>>,
    mut commands: Commands
) {
    for (e, con, input) in &query {
        if input.pressed(PlayerInput::Right) {
            commands.entity(e).try_insert(PlayerLookState::LookingRight);
        }
        if input.pressed(PlayerInput::Left) {
            commands.entity(e).try_insert(PlayerLookState::LookingLeft);
        }

    }
}