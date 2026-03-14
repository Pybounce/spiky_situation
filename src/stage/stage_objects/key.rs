use bevy::{math::Rect, prelude::{Commands, Component}};
use avian2d::prelude::*;

use crate::{common::{death::DeathMarkOnTouch, physics::layers::GamePhysicsLayer}, stage::stage_builder::{stage_asset, stage_creator::StageCreator}};

use super::tiles::PhysicalTileBundle;
use crate::common::triggers::*;

#[derive(Component)]
pub struct Key;


pub struct KeyFactory;

impl KeyFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, atlas_rect: Rect, key_asset: &stage_asset::Key) {
        
        commands.spawn((
            PhysicalTileBundle::new(stage_creator, key_asset.grid_pos, atlas_rect, 0.0, stage_creator.object_tilemap, stage_creator.object_specular_tilemap.into(), CollisionLayers::new(GamePhysicsLayer::StageObject, LayerMask::ALL)),
            Key,
            Trigger {
                trigger_id: key_asset.trigger_id
            },
            TriggerOnTouch,
            DeathMarkOnTouch,
            Sensor
        ));
    }
}
