use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{audio::{PlaySfxEvent, Sfx}, common::{physics::gravity::Gravity, player_input::{PlayerInput, PlayerInputController}}, ground::Grounded, wall::TouchingWall};

use super::{horizontal_movement_controller::AirbourneHorizontalMovementController, jump_controller::{CoyoteGrounded, JumpController}};

#[derive(Component)]
pub struct WallStickable {
    /// Amount of time player sticks to a wall
    pub wall_stick_time: f64
}

#[derive(Component)]
pub struct WallStuck {
    pub touching_wall: TouchingWall,
    /// Time the player started to try getting away from the wall
    pub last_unstuck_time: f64
}


#[derive(Component)]
pub struct WallJumpController {
    /// Force applied when jumping into the wall
    pub force_in: Vec2,
    /// Force applied when jumping away from the wall
    pub force_out: Vec2,
    pub friction_coefficient: f32,
}

pub fn begin_player_wall_jump(
    mut query: Query<(&mut Gravity, &mut LinearVelocity, &mut JumpController, &TouchingWall, &WallJumpController, &AirbourneHorizontalMovementController, &PlayerInputController, &Transform), 
        (Without<Grounded>, Without<CoyoteGrounded>)>,
    time: Res<Time>,
    mut sfx_writer: EventWriter<PlaySfxEvent>
) {
    for (mut g, mut v, mut jc, w, wjc, ahmc, input, t) in &mut query {
        if input.just_pressed(PlayerInput::Jump) {
            v.0 = match w {
                TouchingWall::Left => {
                    if input.pressed(PlayerInput::Left) {
                        wjc.force_out * Vec2::new(-1.0, 1.0)
                    }
                    else {
                        wjc.force_in * Vec2::new(-1.0, 1.0)
                    }
                },
                TouchingWall::Right => {
                    if input.pressed(PlayerInput::Right) {
                        wjc.force_out
                    }
                    else {
                        wjc.force_in
                    }
                },
            }; 
            g.current_force = 0.0;
            jc.last_grounded -= jc.coyote_time; //todo: this sucks but it fixes being able to jump from the ground, and then jump again during coyote time
            jc.last_jump_pressed_time = time.elapsed_secs_f64(); //todo: wrapped??

            sfx_writer.write(PlaySfxEvent {
                sfx: Sfx::PlayerJump,
                translation: t.translation,
            });
        }
    }
} 
pub fn update_wall_stuck_time(
    mut query: Query<(Entity, &mut WallStuck, &AirbourneHorizontalMovementController, &WallStickable, &PlayerInputController)>,
    time: Res<Time>,
    mut commands: Commands
) {
    for (e, mut wall_stuck, mc, wall_stickable, input) in &mut query {
        let push_away = match wall_stuck.touching_wall {
            TouchingWall::Left => input.pressed(PlayerInput::Left),
            TouchingWall::Right => input.pressed(PlayerInput::Right),
        };
        if !push_away {
            wall_stuck.last_unstuck_time = time.elapsed_secs_f64();
        }
        else if time.elapsed_secs_f64() - wall_stuck.last_unstuck_time >= wall_stickable.wall_stick_time {
            commands.entity(e).remove::<WallStuck>();
        }
    }
}

pub fn update_wall_stuck(
    mut query: Query<(&TouchingWall, &mut WallStuck), Changed<TouchingWall>>,
    time: Res<Time>
) {
    for (tw, mut ws) in &mut query {
        match (&tw, &ws.touching_wall) {
            (TouchingWall::Left, TouchingWall::Left) => continue,
            (TouchingWall::Right, TouchingWall::Right) => continue,
            _ => {
                ws.touching_wall = *tw;
                ws.last_unstuck_time = time.elapsed_secs_f64();
            }
        };
    }
}

pub fn add_wall_stuck(
    mut query: Query<(Entity, &TouchingWall), (Without<WallStuck>, Changed<TouchingWall>)>,
    mut commands: Commands,
    time: Res<Time>
) {
    for (e, tw) in &mut query {
        commands.entity(e).try_insert(WallStuck {
            touching_wall: *tw,
            last_unstuck_time: time.elapsed_secs_f64(),
        });
    }
}

pub fn remove_wall_stuck(
    mut query: Query<Entity, (With<WallStuck>, Or<(Without<TouchingWall>, Without<WallStickable>)>)>,
    mut commands: Commands,
) {
    for e in &mut query {
        commands.entity(e).remove::<WallStuck>();
    }
}