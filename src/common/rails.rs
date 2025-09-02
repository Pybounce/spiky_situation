
use std::ops::{Add, Sub};

use bevy::{platform::collections::HashMap, prelude::*};


#[derive(Resource)]
pub struct RailGraph {
    pub rails: HashMap<u32, Vec<Vec3>>
}

#[derive(Component)]
pub struct RailRider {
    pub rail_id: u32,
    pub current_waypoint_index: usize,
    pub reversed: bool
}


pub fn move_rail_riders(
    mut query: Query<(&mut Transform, &mut RailRider)>,
    rail_graph_opt: Option<Res<RailGraph>>,
    time: Res<Time>
) {

    let Some(rail_graph) = rail_graph_opt else { return };

    for (mut transform, mut rider) in &mut query {
        if let Some(rail) = rail_graph.rails.get(&rider.rail_id) {

            let current_target = rail[rider.current_waypoint_index % rail.len()];
            let delta = current_target - transform.translation;
            let dir = delta.normalize_or_zero();
            let speed = 30.0;

            let mut step = dir * speed * time.delta_secs();

            if delta.length() - step.length() <= 0.0 {
                // waypoint reached.
                step = delta;

                let looped_rail = rail.first().unwrap() == rail.last().unwrap();
                if !looped_rail && (rider.current_waypoint_index == rail.len().sub(1) || rider.current_waypoint_index == 0) {
                    rider.reversed = !rider.reversed;
                }

                let mut next_waypoint = if rider.reversed { rider.current_waypoint_index as i32 - 1 } else { rider.current_waypoint_index as i32 + 1 };
                next_waypoint = next_waypoint % rail.len() as i32;
                rider.current_waypoint_index = next_waypoint as usize;
            }

            transform.translation += step;

        }
    }
}