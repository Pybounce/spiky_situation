use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionGroups, Group};

use crate::{common::{death::DeathMarker, triggers::{TriggerEvent, Triggerable}}, stage::stage_builder::{stage_asset::{self}, stage_creator::StageCreator}};

use super::tiles::PhysicalTileBundle;

#[derive(Component)]
pub struct LockBlock;



pub struct LockBlockFactory;

impl LockBlockFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, atlas_rect: Rect, lock_block_asset: &stage_asset::LockBlock) {
        
        commands.spawn((
            PhysicalTileBundle::new(stage_creator, lock_block_asset.grid_pos, atlas_rect, 0.0, stage_creator.object_tilemap, CollisionGroups::new(Group::GROUP_1, Group::ALL)),
            LockBlock,
            Triggerable {
                trigger_id: lock_block_asset.trigger_id
            }
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