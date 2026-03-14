use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{common::{death::DeathMarker, physics::layers::GamePhysicsLayer, triggers::{TriggerEvent, Triggerable}}, ground::Ground, rt_lights::components::{StaticLightOccluder, LightOccluder}, stage::stage_builder::{stage_asset::{self}, stage_creator::StageCreator}};

use super::tiles::PhysicalTileBundle;

#[derive(Component)]
pub struct LockBlock;



pub struct LockBlockFactory;

impl LockBlockFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, atlas_rect: Rect, lock_block_asset: &stage_asset::LockBlock) {
        
        commands.spawn((
            PhysicalTileBundle::new(stage_creator, lock_block_asset.grid_pos, atlas_rect, 0.0, stage_creator.object_tilemap, stage_creator.object_specular_tilemap.into(), CollisionLayers::new(GamePhysicsLayer::Ground, LayerMask::ALL)),
            LockBlock,
            Triggerable {
                trigger_id: lock_block_asset.trigger_id
            },
            Ground,
            LightOccluder::Rect(16.0, 16.0)
        ));
    }
}


pub fn read_lock_block_triggers(
    mut commands: Commands,
    lock_block_triggerables: Query<(Entity, &Triggerable), With<LockBlock>>,
    mut trigger_event_reader: EventReader<TriggerEvent>
) {
    for trigger_event in trigger_event_reader.read() {
        for (entity, triggerable) in &lock_block_triggerables {
            if triggerable.trigger_id == trigger_event.trigger_id {
                commands.entity(entity).try_insert(DeathMarker::default());
            }
        }
    }
}