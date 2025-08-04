use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{local_player::LocalPlayer, player::death::Respawnable, stage::stage_builder::{stage_creator::{StageCreator, TILE_SIZE, TILE_SIZE_HALF}, CurrentStageData}};


#[derive(Component)]
pub struct Checkpoint;

#[derive(Bundle)]
pub struct CheckpointBundle {
    pub checkpoint_marker: Checkpoint,
    pub transform: Transform,
    pub sprite: Sprite,
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub sensor_marker: Sensor,
    pub active_events: ActiveEvents
}

impl CheckpointBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect) -> CheckpointBundle {
        CheckpointBundle {
            checkpoint_marker: Checkpoint,
            rigidbody: RigidBody::Fixed,
            collider: Collider::ball(0.5),
            sensor_marker: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
            transform: Transform {
                scale: Vec3::new(TILE_SIZE, TILE_SIZE, 1.0),
                translation: Vec3::new((grid_pos.x * TILE_SIZE) + TILE_SIZE_HALF, (grid_pos.y * TILE_SIZE) + TILE_SIZE_HALF, 0.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                rect: Some(atlas_rect),
                image: stage_creator.object_tilemap.clone(),
                ..default()
            },
        }
    }
}

pub fn check_checkpoint_reached(
    checkpoint_query: Query<(Entity, &Transform), With<Checkpoint>>,
    mut player_query: Query<(&mut Respawnable, &CollidingEntities), With<LocalPlayer>>,
    mut stage_data: Option<ResMut<CurrentStageData>>,
    mut commands: Commands
) {
    if let Some(stage_data) = &mut stage_data {
        for (mut r, ce) in &mut player_query {
            for colliding_entity in ce.iter() {
                if let Ok((e, t)) = checkpoint_query.get(colliding_entity) {
                    r.translation = t.translation;
                    stage_data.spawn_translation = t.translation;
                    commands.entity(e).despawn();
                }
            }
        }
    }

}