
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{ground::Grounded, wall::TouchingWall};

use super::wall_jump_controller::WallStuck;

#[derive(Component)]
pub struct GroundedHorizontalMovementController {
    pub left_key: KeyCode,
    pub right_key: KeyCode,
    pub acceleration: f32,
    pub max_speed: f32,
    pub deceleration: f32
}


pub fn move_ground_horizontal_controller(
    mut query: Query<(&mut Velocity, &GroundedHorizontalMovementController), With<Grounded>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    for (mut vel, con) in &mut query {

        let mut change: f32 = 0.0;
        if input.pressed(con.right_key) {
            change += con.acceleration * time.delta_secs();
        }
        if input.pressed(con.left_key) {
            change -= con.acceleration * time.delta_secs();
        }

        vel.linvel.x += change;

        if vel.linvel.x.abs() > con.max_speed {
           // vel.linvel.x -= 0.4 * (vel.linvel.x.abs() - con.max_speed.abs()).powi(2) * vel.linvel.x.signum() * time.delta_seconds();
           let friction = change.abs();
           let diff = vel.linvel.x.abs() - con.max_speed;
           vel.linvel.x -= vel.linvel.x.signum() * diff.abs().min(friction);
        } 
        if change.abs() < 0.01 {
            let friction = con.deceleration * time.delta_secs();
            vel.linvel.x -= vel.linvel.x.signum() * vel.linvel.x.abs().min(friction);
        }
    }
}



#[derive(Component)]
pub struct AirbourneHorizontalMovementController {
    pub left_key: KeyCode,
    pub right_key: KeyCode,
    pub acceleration: f32,
    pub max_speed: f32,
    pub deceleration: f32
}


pub fn move_airbourne_horizontal_controller(
    mut query: Query<(&mut Velocity, &AirbourneHorizontalMovementController, Option<&WallStuck>), Without<Grounded>>,    //todo: need an airbourne state, right now there are seaprate states for jumping
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    for (mut vel, con, ws_opt) in &mut query {

        let mut change: f32 = 0.0;
        if input.pressed(con.right_key) {
            change += con.acceleration * time.delta_secs();
        }
        if input.pressed(con.left_key) {
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

        vel.linvel.x += change;
        
        if vel.linvel.x.abs() > con.max_speed {
            //vel.linvel.x -= 0.4 * (vel.linvel.x.abs() - con.max_speed.abs()).powi(2) * vel.linvel.x.signum() * time.delta_seconds();
            let friction = change.abs();
            let diff = vel.linvel.x.abs() - con.max_speed;
            vel.linvel.x -= vel.linvel.x.signum() * diff.abs().min(friction);
        } 
        if change.abs() < 0.01 {
            let friction = con.deceleration * time.delta_secs();
            vel.linvel.x -= vel.linvel.x.signum() * vel.linvel.x.abs().min(friction);
        }

    }
}