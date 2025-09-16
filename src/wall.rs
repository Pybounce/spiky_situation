use bevy::prelude::*;
use avian2d::prelude::*;

use crate::common::physics::layers::GamePhysicsLayer;


#[derive(Component)]
pub struct Wall;
#[derive(Component)]
pub struct Wallable;

/// varients describe the side of the wall, not which side the wall is on
#[derive(Component, Copy, Clone, PartialEq, Eq)]
pub enum TouchingWall {
    Left,
    Right
}


pub fn check_touching_wall(
    mut commands: Commands,
    mut wallable_query: Query<(Entity, &mut Transform, &mut LinearVelocity, Option<&TouchingWall>), With<Wallable>>,
    _wall_query: Query<(), With<Wall>>,
    spatial: SpatialQuery,
) {
    for (entity, mut transform, mut velocity, tw_opt) in &mut wallable_query {
        let mut new_left_collision = false;
        let mut new_right_collision = false;

        let spatial_filter = SpatialQueryFilter::from_mask(GamePhysicsLayer::Ground).with_excluded_entities([entity]);



        let raycast_buffer = 2.0;
        let raycast_length = transform.scale.y / 2.0;
        let solid = false;
        let ray_count = 3;

        let mut ray_pos = Vec2::new(transform.translation.x, transform.translation.y - (transform.scale.y / 2.0));
        for _ in 0..ray_count {
            ray_pos.y += transform.scale.y / (ray_count + 1) as f32;

            if let Some(hit) = spatial.cast_ray(ray_pos, Dir2::X, raycast_length + raycast_buffer, solid, &spatial_filter) {
                new_left_collision = true;
                if hit.distance <= raycast_length {
                    velocity.0.x = velocity.0.x.min(0.0);
                    transform.translation.x -= raycast_length - hit.distance;
                    break;
                }
            }
        }

        ray_pos = Vec2::new(transform.translation.x, transform.translation.y - (transform.scale.y / 2.0));
        for _ in 0..ray_count {
            ray_pos.y += transform.scale.y / (ray_count + 1) as f32;

            if let Some(hit) = spatial.cast_ray(ray_pos, Dir2::NEG_X, raycast_length + raycast_buffer , solid, &spatial_filter) {
                new_right_collision = true;
                if hit.distance <= raycast_length {
                    velocity.0.x = velocity.0.x.max(0.0);
                    transform.translation.x += raycast_length - hit.distance;
                    break;
                }
            }
        }
        // if it's the new collision is already set, continue.
        if let Some(tw) = tw_opt {
            match tw {
                TouchingWall::Left => {
                    if new_left_collision { continue; }
                },
                TouchingWall::Right => {
                    if new_right_collision { continue; }
                },
            }
        }

        if new_left_collision {
            commands.entity(entity).try_insert(TouchingWall::Left);
        }
        else if new_right_collision {
            commands.entity(entity).try_insert(TouchingWall::Right);
        }
        else if !new_right_collision && !new_left_collision {
            commands.entity(entity).remove::<TouchingWall>();
        }
    }
}


