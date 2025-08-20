use bevy::{math::Rect, prelude::{Commands, Component, Entity, Query, Res, Without}, time::{Time, Timer, TimerMode}};
use bevy_rapier2d::prelude::{CollidingEntities, CollisionGroups, Group};

use crate::{common::{animated_sprite::SpriteAnimator, death::DeathMarker}, ground::Ground, stage::stage_builder::{stage_asset, stage_creator::StageCreator}};

use super::tiles::PhysicalTileBundle;

#[derive(Component)]
pub struct PhantomBlock {
    timer: Timer,
    currently_active: bool
}

pub struct PhantomBlockFactory;

impl PhantomBlockFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, atlas_rects: Vec<Rect>, phantom_block_asset: &stage_asset::PhantomBlock) {
        
        let animation_frame_delta: u128 = 50;

        commands.spawn((
            PhysicalTileBundle::new(stage_creator, phantom_block_asset.grid_pos, atlas_rects[0], 0.0, stage_creator.object_tilemap, CollisionGroups::new(Group::GROUP_1, Group::ALL)),
            PhantomBlock {
                timer: Timer::from_seconds(atlas_rects.len() as f32 * (animation_frame_delta as f32 / 1000.0), TimerMode::Once),
                currently_active: false
            },
            SpriteAnimator::new_non_repeating(animation_frame_delta, atlas_rects),
            Ground,
        ));
    }
}

pub fn check_phantom_block_touched(
    colliding_query: Query<&CollidingEntities, Without<PhantomBlock>>,
    mut phantom_query: Query<(&mut PhantomBlock, &mut SpriteAnimator)>
) {
    for colliding_entities in &colliding_query {

        for colliding_entity in colliding_entities.iter() {
            if let Ok((mut pb, mut sa)) = phantom_query.get_mut(colliding_entity) {
                pb.currently_active = true;
                sa.play();
            }
        }
    }
}

pub fn tick_phantom_block(
    mut phantom_query: Query<(Entity, &mut PhantomBlock)>,
    time: Res<Time>,
    mut commands: Commands
) {
    for (entity, mut pb) in &mut phantom_query {
        if pb.currently_active {
            pb.timer.tick(time.delta());
            if pb.timer.just_finished() {
                commands.entity(entity).try_insert(DeathMarker::default());
            }
        }
    }
}