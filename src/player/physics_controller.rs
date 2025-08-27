
use bevy::prelude::*;
use avian2d::prelude::*;

#[derive(Component)]
pub struct PhysicsController {
    pub max_velocity: Vec2,
    pub min_velocity: Vec2
}

pub fn apply_physics_controller_limits(
    mut query: Query<(&mut LinearVelocity, &PhysicsController)>
) {
    for (mut v, pc) in &mut query {
        v.0.x = v.0.x.max(pc.min_velocity.x).min(pc.max_velocity.x);
        v.0.y = v.0.y.max(pc.min_velocity.y).min(pc.max_velocity.y);
    }
}