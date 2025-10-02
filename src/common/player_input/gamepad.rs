
use bevy::{input::gamepad::{GamepadConnection, GamepadEvent}, prelude::*};

use crate::common::player_input::{PlayerInput, PlayerInputController};


#[derive(Component)]
pub struct PlayerGamepadInput {
    pub l_stick_deadzone: f32,
    pub move_left_button: GamepadButton,
    pub move_right_button: GamepadButton,
    pub jump_button: GamepadButton,
    pub dash_button: GamepadButton
}

pub fn handle_player_gamepad_input(
    gamepad_query: Query<&Gamepad>,
    mut player_input_query: Query<(&mut PlayerInputController, &PlayerGamepadInput)>
) {
    let Ok((mut player_input, player_gamepad_controller)) = player_input_query.single_mut() else { return; };

    for gamepad in gamepad_query {
        if let Some(lx_axis) = gamepad.analog().get(GamepadAxis::LeftStickX) {
            if lx_axis < -player_gamepad_controller.l_stick_deadzone {
                player_input.press(PlayerInput::Left);
            }
            else if lx_axis > player_gamepad_controller.l_stick_deadzone {
                player_input.press(PlayerInput::Right);
            }
        }
    
        if gamepad.pressed(player_gamepad_controller.move_left_button) {
            player_input.press(PlayerInput::Left);
        }
        if gamepad.pressed(player_gamepad_controller.move_right_button) {
            player_input.press(PlayerInput::Right);
        }
    
        if gamepad.pressed(player_gamepad_controller.jump_button) {
            player_input.press(PlayerInput::Jump);
        }
        if gamepad.pressed(player_gamepad_controller.dash_button) {
            player_input.press(PlayerInput::Dash);
        }
    }




}