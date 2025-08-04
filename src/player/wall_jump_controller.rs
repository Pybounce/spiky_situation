use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{common::physics::gravity::Gravity, ground::Grounded, wall::TouchingWall};

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
    mut query: Query<(&mut Gravity, &mut Velocity, &mut JumpController, &TouchingWall, &WallJumpController, &AirbourneHorizontalMovementController), 
        (Without<Grounded>, Without<CoyoteGrounded>)>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>
) {
    for (mut g, mut v, mut jc, w, wjc, ahmc) in &mut query {
        if input.just_pressed(jc.key) {
            v.linvel = match w {
                TouchingWall::Left => {
                    if input.pressed(ahmc.left_key) {
                        wjc.force_out * Vec2::new(-1.0, 1.0)
                    }
                    else {
                        wjc.force_in * Vec2::new(-1.0, 1.0)
                    }
                },
                TouchingWall::Right => {
                    if input.pressed(ahmc.right_key) {
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
        }
    }
} 
pub fn update_wall_stuck_time(
    mut query: Query<(Entity, &mut WallStuck, &AirbourneHorizontalMovementController, &WallStickable)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands
) {
    for (e, mut wall_stuck, mc, wall_stickable) in &mut query {
        let push_away_key = match wall_stuck.touching_wall {
            TouchingWall::Left => mc.left_key,
            TouchingWall::Right => mc.right_key,
        };
        if !input.pressed(push_away_key) {
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