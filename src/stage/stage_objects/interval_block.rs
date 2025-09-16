use bevy::{math::Rect, prelude::{Commands, Component, Entity, Query, Res, With}, time::{Time, Timer, TimerMode}};
use avian2d::prelude::*;

use crate::{common::{animated_sprite::SpriteAnimator, physics::layers::GamePhysicsLayer}, ground::Ground, obstacles::InstantKiller, stage::stage_builder::{stage_asset, stage_creator::{StageCreator, TILE_SIZE}}};

use super::tiles::PhysicalTileBundle;

//TODO: Like the PhantomBlock, I can add a timer that runs once for the duraction of frame delay * animation len
// Then I can disable/enable colliders then, but it's not a big deal

#[derive(Component)]
pub struct IntervalBlock {
    timer: Timer,  
    currently_active: bool
}

pub struct IntervalBlockFactory;

impl IntervalBlockFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, atlas_rects: Vec<Rect>, interval_block_asset: &stage_asset::IntervalBlock) {
        
        commands.spawn((
            PhysicalTileBundle::new(stage_creator, interval_block_asset.grid_pos, atlas_rects[0], 0.0, stage_creator.object_tilemap, CollisionLayers::new(GamePhysicsLayer::Ground, LayerMask::ALL)),
            IntervalBlock {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                currently_active: interval_block_asset.is_active
            },
            SpriteAnimator::new_non_repeating(50, atlas_rects),
            Ground,
        ));
    }
}

pub fn tick_interval_blocks(
    mut query: Query<(Entity, &mut IntervalBlock, &mut SpriteAnimator)>,
    time: Res<Time>,
    mut commands: Commands
) {
    for (e, mut interval_block, mut sprite_anim) in &mut query {
        interval_block.timer.tick(time.delta());
        if interval_block.timer.just_finished() {
            interval_block.currently_active = !interval_block.currently_active;
            if interval_block.currently_active {
                sprite_anim.play_reverse();
                commands.entity(e).try_insert(Collider::rectangle(TILE_SIZE, TILE_SIZE));
                commands.entity(e).try_insert(InstantKiller);
            }
            if !interval_block.currently_active {
                sprite_anim.play();
                commands.entity(e).remove::<Collider>();
            }
        }
    }
}

pub fn stop_interval_block_crush(query: Query<Entity, (With<InstantKiller>, With<IntervalBlock>)>, mut commands: Commands) {
    for e in &query {
        commands.entity(e).remove::<InstantKiller>();
    }
}