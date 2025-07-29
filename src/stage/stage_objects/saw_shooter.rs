use bevy::{math::{Quat, Rect, Vec2, Vec3}, prelude::{Commands, Component, Entity, Query, Res, With}, sprite::{Sprite, SpriteBundle}, time::{Time, Timer, TimerMode}, transform::{bundles::TransformBundle, components::Transform}, utils::default};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionGroups, GravityScale, Group, LockedAxes, RigidBody, Sensor, Velocity};

use crate::{common::{animated_sprite::SpriteAnimator, physics::fragile::{Fragile, FragileShield}}, ground::Ground, obstacles::InstantKiller, stage::stage_builder::{stage_asset, stage_creator::{get_object_tilemap_rect_from_index, ObjectAtlasIndices, StageCreator, TILE_SIZE_HALF}, StageAssets}};

use super::tiles::PhysicalTileBundle;

#[derive(Component)]
pub struct SawShooter {
    timer: Timer
}
#[derive(Component)]
pub struct SmallSaw;

#[derive(Component)]
pub struct SawShooterBlock;

pub struct SawShooterFactory;

impl SawShooterFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, atlas_rects: Vec<Rect>, saw_shooter_block_asset: &stage_asset::SawShooterBlock) {
        
        commands.spawn((
            PhysicalTileBundle::new(stage_creator, saw_shooter_block_asset.grid_pos, atlas_rects[0], saw_shooter_block_asset.rotation, stage_creator.object_tilemap, CollisionGroups::new(Group::GROUP_1, Group::ALL)),
            SawShooter {
                timer: Timer::from_seconds(3.0, TimerMode::Repeating)
            },
            SawShooterBlock,
            SpriteAnimator::new_non_repeating(50, atlas_rects),
            Ground,
        ));
    }
}

pub fn tick_saw_shooters(
    mut query: Query<(&mut SawShooter, &Transform)>,
    time: Res<Time>,
    mut commands: Commands,
    stage_assets_opt: Option<Res<StageAssets>>
) {
    if stage_assets_opt.is_none() { return; }
    let stage_assets = stage_assets_opt.unwrap();

    let atlas_rects = vec![
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::SawProjectile0),
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::SawProjectile1),
        get_object_tilemap_rect_from_index(ObjectAtlasIndices::SawProjectile2)
    ];
    for (mut saw_shooter, transform) in &mut query {
        saw_shooter.timer.tick(time.delta());
        if saw_shooter.timer.just_finished() {
            commands.spawn((SmallSaw, Fragile, FragileShield, InstantKiller, Collider::ball(0.3 * 16.0), Velocity::linear((transform.rotation * Vec2::new(0.0, 100.0).extend(0.0)).truncate()),
            Sensor,
            RigidBody::Dynamic,
            GravityScale(0.0),
            ActiveEvents::COLLISION_EVENTS,
            LockedAxes::ROTATION_LOCKED,
            SpriteBundle {
                texture: stage_assets.stage_objects_handle.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    rect: Some(atlas_rects[0]),
                    ..default()
                },
                transform: Transform::from_translation(transform.translation - Vec3::new(0.0, 0.0, 1.0)),
                ..default()
            }, 
            SpriteAnimator::new(30, atlas_rects.clone())));
        }
    }
}
/*


SpriteBundle {
                transform: Transform {
                    rotation: Quat::from_rotation_z(tile_rotation),
                    translation: Vec3::new((grid_pos.x * TILE_SIZE) + TILE_SIZE_HALF, (grid_pos.y * TILE_SIZE) + TILE_SIZE_HALF, 0.0),
                    ..default()
                },
                texture: image_handle.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    rect: Some(atlas_rect),
                    ..default()
                },
                ..default()
            }
            
            
            
            */
//pub fn animate_saw_shooter_blocks(
//    mut query: Query<(Entity, &mut SawShooter, &mut SpriteAnimator)>,
//    time: Res<Time>,
//    mut commands: Commands
//) {
//    for (e, mut interval_block, mut sprite_anim) in &mut query {
//        interval_block.timer.tick(time.delta());
//        if interval_block.timer.just_finished() {
//            sprite_anim.play();
//            commands.entity(e).try_insert(Collider::cuboid(TILE_SIZE_HALF, TILE_SIZE_HALF));
//            commands.entity(e).try_insert(InstantKiller);
//        }
//    }
//}
