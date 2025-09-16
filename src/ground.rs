use bevy::prelude::*;
use avian2d::prelude::*;

use crate::common::physics::layers::GamePhysicsLayer;



#[derive(Component)]
pub struct Ground;
#[derive(Component)]
pub struct Groundable;

#[derive(Component)]
pub struct Grounded;

pub fn check_grounded(
    mut commands: Commands,
    mut wallable_query: Query<(Entity, &mut Transform, &mut LinearVelocity), With<Groundable>>,
    spatial: SpatialQuery,
) {
    for (entity, mut transform, mut velocity) in &mut wallable_query {
        let mut ground_collision = false;

        let spatial_filter = SpatialQueryFilter::from_mask(GamePhysicsLayer::Ground).with_excluded_entities([entity]);

        let raycast_buffer = 2.0;
        let raycast_length = transform.scale.y / 2.0;
        let solid = false;
        let ray_count = 3;

        let mut ray_pos = Vec2::new(transform.translation.x - (transform.scale.x / 2.0), transform.translation.y);
        for _ in 0..ray_count {
            ray_pos.x += transform.scale.x / (ray_count + 1) as f32;
            if let Some(hit) = spatial.cast_ray(ray_pos, Dir2::Y, raycast_length + raycast_buffer, solid, &spatial_filter) {
                if hit.distance <= raycast_length {
                    velocity.0.y = velocity.0.y.min(0.0);
                    transform.translation.y -= raycast_length - hit.distance;
                    break;
                }
            }
        }
        ray_pos = Vec2::new(transform.translation.x - (transform.scale.x / 2.0), transform.translation.y);
        for _ in 0..ray_count {
            ray_pos.x += transform.scale.x / (ray_count + 1) as f32;

            if let Some(hit) = spatial.cast_ray(ray_pos, Dir2::NEG_Y, raycast_length + raycast_buffer, solid, &spatial_filter) {
                ground_collision = true;

                if hit.distance <= raycast_length {
                    velocity.0.y = velocity.0.y.max(0.0);
                    transform.translation.y += raycast_length - hit.distance;
                    break;
                }
            }
        }
        if ground_collision {
            commands.entity(entity).try_insert(Grounded);
        }
        else if !ground_collision {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}





