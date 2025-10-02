use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{common::{physics::layers::GamePhysicsLayer, splat::SplatProvider}, obstacles::InstantKiller, rt_lights::components::LightOccluder, stage::stage_builder::stage_creator::{StageCreator, TILE_SIZE, TILE_SIZE_HALF}};

use super::tiles::TileBundle;

#[derive(Component)]
pub struct Spike;



pub struct SpikeFactory;

impl SpikeFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect, rotation: f32) {
        let mut mask = LayerMask(GamePhysicsLayer::StageObject.to_bits());
        mask.add(GamePhysicsLayer::Player.to_bits());
        
        commands.spawn((
            TileBundle::new(stage_creator, grid_pos, atlas_rect, rotation, stage_creator.object_tilemap),
            RigidBody::Static,
            InstantKiller,
            Spike,
            SplatProvider {
                translation_offset: Vec2::new(0.0, -(TILE_SIZE_HALF * 0.2)),
            },
            children![(
                Transform::from_translation(Vec3::new(0.0, -TILE_SIZE_HALF * 0.2, 0.0)),
                Collider::rectangle(TILE_SIZE * 0.8, TILE_SIZE * 0.8),
                CollisionLayers::new(GamePhysicsLayer::StageObject, mask),
                CollisionEventsEnabled,
            )]
        ));

    }
    pub fn spawn_editor_icon(commands: &mut Commands, grid_pos: IVec2, rotation: f32, atlas: &Handle<Image>, atlas_rect: Rect) -> Entity {
        commands.spawn((
            Transform {
                rotation: Quat::from_rotation_z(rotation),
                translation: Vec3::new((grid_pos.x as f32 * TILE_SIZE) + TILE_SIZE_HALF, (grid_pos.y as f32 * TILE_SIZE) + TILE_SIZE_HALF, 0.0),
                ..default()
            },
            Sprite {
                image: atlas.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                rect: Some(atlas_rect),
                ..default()
            }
        )).id()
    }
}
