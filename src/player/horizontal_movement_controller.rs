
use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{common::player_input::{PlayerInput, PlayerInputController}, ground::Grounded, wall::TouchingWall};

use super::wall_jump_controller::WallStuck;

#[derive(Component)]
pub struct GroundedHorizontalMovementController {
    pub acceleration: f32,
    pub max_speed: f32,
    pub deceleration: f32
}


pub fn move_ground_horizontal_controller(
    mut query: Query<(&mut LinearVelocity, &GroundedHorizontalMovementController, &PlayerInputController), With<Grounded>>,
    time: Res<Time>
) {
    for (mut vel, con, input) in &mut query {

        let mut change: f32 = 0.0;
        if input.pressed(PlayerInput::Right) {
            change += con.acceleration * time.delta_secs();
        }
        if input.pressed(PlayerInput::Left) {
            change -= con.acceleration * time.delta_secs();
        }

        vel.0.x += change;

        if vel.0.x.abs() > con.max_speed {
           // vel.linvel.x -= 0.4 * (vel.linvel.x.abs() - con.max_speed.abs()).powi(2) * vel.linvel.x.signum() * time.delta_seconds();
           let friction = change.abs();
           let diff = vel.0.x.abs() - con.max_speed;
           vel.0.x -= vel.0.x.signum() * diff.abs().min(friction);
        } 
        if change.abs() < 0.01 {
            let friction = con.deceleration * time.delta_secs();
            vel.0.x -= vel.0.x.signum() * vel.0.x.abs().min(friction);
        }
    }
}



#[derive(Component)]
pub struct AirbourneHorizontalMovementController {
    pub acceleration: f32,
    pub max_speed: f32,
    pub deceleration: f32
}


pub fn move_airbourne_horizontal_controller(
    mut query: Query<(&mut LinearVelocity, &AirbourneHorizontalMovementController, Option<&WallStuck>, &PlayerInputController), Without<Grounded>>,    //todo: need an airbourne state, right now there are seaprate states for jumping
    time: Res<Time>
) {
    for (mut vel, con, ws_opt, input) in &mut query {

        let mut change: f32 = 0.0;
        if input.pressed(PlayerInput::Right) {
            change += con.acceleration * time.delta_secs();
        }
        if input.pressed(PlayerInput::Left) {
            change -= con.acceleration * time.delta_secs();
        }



        if let Some(ws) = ws_opt {
            match ws.touching_wall {
                TouchingWall::Left => {
                    if change < 0.0 { change = 0.0; }
                },
                TouchingWall::Right => {
                    if change > 0.0 { change = 0.0; }
                },
            };
        }

        vel.0.x += change;
        
        if vel.0.x.abs() > con.max_speed {
            //vel.linvel.x -= 0.4 * (vel.linvel.x.abs() - con.max_speed.abs()).powi(2) * vel.linvel.x.signum() * time.delta_seconds();
            let friction = change.abs();
            let diff = vel.0.x.abs() - con.max_speed;
            vel.0.x -= vel.0.x.signum() * diff.abs().min(friction);
        } 
        if change.abs() < 0.01 {
            let friction = con.deceleration * time.delta_secs();
            vel.0.x -= vel.0.x.signum() * vel.0.x.abs().min(friction);
        }

    }
}