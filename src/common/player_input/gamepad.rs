
use bevy::{input::gamepad::{GamepadConnection, GamepadEvent}, prelude::*};

use crate::common::player_input::{PlayerInput, PlayerInputController};


#[derive(Resource)]
pub struct PlayerGamepad(pub Entity);


#[derive(Component)]
pub struct PlayerGamepadInput {
    pub l_stick_deadzone: f32,
    pub move_left_button: GamepadButton,
    pub move_right_button: GamepadButton,
    pub jump_button: GamepadButton,
    pub dash_button: GamepadButton
}

pub fn handle_gamepad_connections(
    mut commands: Commands,
    gamepad_opt: Option<Res<PlayerGamepad>>,
    mut evr_gamepad: EventReader<GamepadEvent>
) {
    for ev in evr_gamepad.read() {
        let GamepadEvent::Connection(ev_conn) = ev else { continue; };

        match &ev_conn.connection {
            GamepadConnection::Connected {..} => {
                if gamepad_opt.is_none() {
                    commands.insert_resource(PlayerGamepad(ev_conn.gamepad));
                }
            }
            GamepadConnection::Disconnected => {
                if let Some(PlayerGamepad(current_gamepad)) = gamepad_opt.as_deref() {
                    if *current_gamepad == ev_conn.gamepad {
                        commands.remove_resource::<PlayerGamepad>();
                    }
                }
            }
        }
    }
}

pub fn handle_player_gamepad_input(
    gamepad_query: Query<&Gamepad>,
    gamepad_opt: Option<Res<PlayerGamepad>>,
    mut player_input_query: Query<(&mut PlayerInputController, &PlayerGamepadInput)>
) {
    let Some(gamepad_entity) = gamepad_opt else { return; };
    let Ok(gamepad) = gamepad_query.get(gamepad_entity.0) else { return; };
    let Ok((mut player_input, player_gamepad_controller)) = player_input_query.single_mut() else { return; };

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