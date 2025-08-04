use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::ground::Grounded;

#[derive(Component)]
pub struct Gravity {
    pub max_force: f32,
    pub current_force: f32,
    pub acceleration: f32
}

pub fn simulate_gravity(
    mut query: Query<(&mut Velocity, &mut Gravity), Without<Grounded>>,
    time: Res<Time>
) {
    for (mut v, mut g) in &mut query {
        g.current_force += g.acceleration * time.delta_secs();
        g.current_force = g.current_force.min(g.max_force);

        v.linvel.y -= g.current_force * time.delta_secs();
    }
}