use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


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
    mut wallable_query: Query<(Entity, &mut Transform, &mut Velocity, Option<&TouchingWall>), With<Wallable>>,
    _wall_query: Query<(), With<Wall>>,
    rapier_write_context: WriteRapierContext
) {
    let rapier_context = rapier_write_context.single().unwrap();
    for (entity, mut transform, mut velocity, tw_opt) in &mut wallable_query {
        let mut new_left_collision = false;
        let mut new_right_collision = false;


        let filter = QueryFilter::new()
        .exclude_sensors()
        .exclude_rigid_body(entity)
        .groups(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1));




        let raycast_buffer = 2.0;
        let raycast_length = transform.scale.y / 2.0;
        let solid = false;
        let ray_count = 3;

        let mut ray_pos = Vec2::new(transform.translation.x, transform.translation.y - (transform.scale.y / 2.0));
        for _ in 0..ray_count {
            ray_pos.y += transform.scale.y / (ray_count + 1) as f32;

            if let Some((_entity, toi)) = rapier_context.cast_ray(ray_pos, Vec2::new(1.0, 0.0), raycast_length + raycast_buffer, solid, filter) {
                new_left_collision = true;
                if toi <= raycast_length {
                    velocity.linvel.x = velocity.linvel.x.min(0.0);
                    transform.translation.x -= raycast_length - toi;
                    break;
                }
            }
        }

        ray_pos = Vec2::new(transform.translation.x, transform.translation.y - (transform.scale.y / 2.0));
        for _ in 0..ray_count {
            ray_pos.y += transform.scale.y / (ray_count + 1) as f32;

            if let Some((_entity, toi)) = rapier_context.cast_ray(ray_pos, Vec2::new(-1.0, 0.0), raycast_length + raycast_buffer , solid, filter) {
                new_right_collision = true;
                if toi <= raycast_length {
                    velocity.linvel.x = velocity.linvel.x.max(0.0);
                    transform.translation.x += raycast_length - toi;
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


