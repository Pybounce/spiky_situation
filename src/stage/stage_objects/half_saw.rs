
use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{common::{animated_sprite::SpriteAnimator, offset_mover::OffsetMover, physics::layers::GamePhysicsLayer, splat::SplatProvider}, obstacles::InstantKiller, stage::stage_builder::{stage_asset, stage_creator::{StageCreator, TILE_SIZE_HALF}}};

use super::tiles::TileBundle;


#[derive(Component)]
pub struct HalfSaw;

pub struct SawFactory;

impl SawFactory {
    pub fn spawn_half(commands: &mut Commands, stage_creator: &StageCreator, atlas_rects: Vec<Rect>, saw_asset: &stage_asset::HalfSaw) {
        let mut mask = LayerMask(GamePhysicsLayer::StageObject.to_bits());
        mask.add(GamePhysicsLayer::Player.to_bits());
        
        let mut parent_e = commands.spawn((
            TileBundle::new(stage_creator, saw_asset.grid_pos, atlas_rects[0], saw_asset.rotation, stage_creator.object_tilemap, stage_creator.object_specular_tilemap.into()),
            SpriteAnimator::new(50, atlas_rects),
            InstantKiller,
            HalfSaw,
            SplatProvider {
                translation_offset: Vec2::new(0.0, -TILE_SIZE_HALF),
            },
            RigidBody::Static,
            children![(
                Transform::from_translation(Vec3::new(0.0, -TILE_SIZE_HALF, 0.0)),
                Collider::circle(TILE_SIZE_HALF * 0.9),
                CollisionLayers::new(GamePhysicsLayer::StageObject, mask),
                CollisionEventsEnabled,
            )]
        ));


        match &saw_asset.movement_path_opt {
            Some(mp) => { parent_e.insert(OffsetMover::new_from_grid(&mp.grid_offsets, mp.speed)); },
            None => (),
        };

    }
}