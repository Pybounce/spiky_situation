
use bevy::prelude::*;

use crate::common::player_input::{PlayerInput, PlayerInputController};

#[derive(Component)]
pub struct PlayerKeyboardInput {
    pub move_left_key: KeyCode,
    pub move_right_key: KeyCode,
    pub jump_key: KeyCode,
    pub dash_key: KeyCode
}



pub fn handle_player_keyboard_input(
    input: Res<ButtonInput<KeyCode>>,
    mut player_input_query: Query<(&mut PlayerInputController, &PlayerKeyboardInput)>
) {
    let Ok((mut player_input, keyboard_input)) = player_input_query.single_mut() else { return; };

    if input.pressed(keyboard_input.move_left_key) {
        player_input.press(PlayerInput::Left);
    }
    if input.pressed(keyboard_input.move_right_key) {
        player_input.press(PlayerInput::Right);
    }
    if input.pressed(keyboard_input.jump_key) {
        player_input.press(PlayerInput::Jump);
    }
    if input.pressed(keyboard_input.dash_key) {
        player_input.press(PlayerInput::Dash);
    }


}