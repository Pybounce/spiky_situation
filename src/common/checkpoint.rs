use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{common::physics::avian_ex::ManyCollidingEntities, lit_sprite::global_components::LitSprite, local_player::LocalPlayer, player::death::Respawnable, stage::stage_builder::{CurrentStageData, stage_creator::{StageCreator, TILE_SIZE, TILE_SIZE_HALF}}};


#[derive(Component)]
pub struct Checkpoint;

#[derive(Bundle)]
pub struct CheckpointBundle {
    pub checkpoint_marker: Checkpoint,
    pub transform: Transform,
    pub sprite: LitSprite,
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub sensor_marker: Sensor,
    pub collision_events_enables: CollisionEventsEnabled
}

impl CheckpointBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect) -> CheckpointBundle {
        CheckpointBundle {
            checkpoint_marker: Checkpoint,
            rigidbody: RigidBody::Static,
            collider: Collider::circle(0.5),
            sensor_marker: Sensor,
            collision_events_enables: CollisionEventsEnabled,
            transform: Transform {
                scale: Vec3::new(TILE_SIZE, TILE_SIZE, 1.0),
                translation: Vec3::new((grid_pos.x * TILE_SIZE) + TILE_SIZE_HALF, (grid_pos.y * TILE_SIZE) + TILE_SIZE_HALF, 0.0),
                ..default()
            },
            sprite: LitSprite {
                size: Vec2::new(1.0, 1.0),
                rect: Some(atlas_rect),
                albedo_texture: stage_creator.object_tilemap.clone().into(),
                ..default()
            },
        }
    }
}

pub fn check_checkpoint_reached(
    checkpoint_query: Query<(Entity, &Transform), With<Checkpoint>>,
    mut player_query: Query<(&mut Respawnable, &ManyCollidingEntities), With<LocalPlayer>>,
    mut stage_data: Option<ResMut<CurrentStageData>>,
    mut commands: Commands
) {
    if let Some(stage_data) = &mut stage_data {
        for (mut r, ce) in &mut player_query {
            for colliding_entity in ce.iter() {
                if let Ok((e, t)) = checkpoint_query.get(*colliding_entity) {
                    r.translation = t.translation;
                    stage_data.spawn_translation = t.translation;
                    commands.entity(e).despawn();
                }
            }
        }
    }

}