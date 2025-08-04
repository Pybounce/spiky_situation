
use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionGroups, Group, RigidBody};

use crate::{common::{animated_sprite::SpriteAnimator, offset_mover::OffsetMover}, obstacles::InstantKiller, stage::stage_builder::{stage_asset, stage_creator::{StageCreator, TILE_SIZE_HALF}}};

use super::{tiles::TileBundle, StageObject};


#[derive(Component)]
pub struct HalfSaw;

pub struct SawFactory;

impl SawFactory {
    pub fn spawn_half(commands: &mut Commands, stage_creator: &StageCreator, atlas_rects: Vec<Rect>, saw_asset: &stage_asset::HalfSaw) {
        
        let mut e = commands.spawn((
            TileBundle::new(stage_creator, saw_asset.grid_pos, atlas_rects[0], saw_asset.rotation, stage_creator.object_tilemap),
            SpriteAnimator::new(50, atlas_rects),
        ));
        e.with_children(|parent| {
            parent.spawn((
                Collider::ball(TILE_SIZE_HALF * 0.9),
                Transform::from_xyz(0.0, -TILE_SIZE_HALF, 0.0),
                CollisionGroups::new(Group::GROUP_2, Group::ALL),
                ActiveEvents::COLLISION_EVENTS,
                RigidBody::Fixed,
                InstantKiller,
                HalfSaw,
                StageObject,
            ));
        });
        match &saw_asset.movement_path_opt {
            Some(mp) => { e.insert(OffsetMover::new_from_grid(&mp.grid_offsets, mp.speed)); },
            None => (),
        };
        

    }
}