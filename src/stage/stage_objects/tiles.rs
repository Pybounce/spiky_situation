
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{ground::Ground, stage::{stage_builder::stage_creator::{StageCreator, TILE_SIZE, TILE_SIZE_HALF}, stage_objects::StageObject}, wall::Wall};



#[derive(Bundle)]
pub struct TileBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    stage_marker: StageObject
}

#[derive(Bundle)]
pub struct PhysicalTileBundle {
    pub tile_bundle: TileBundle,
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub restitution: Restitution,
    pub friction: Friction,
    pub gravity_scale: GravityScale,
    pub active_events: ActiveEvents,
    pub collision_groups: CollisionGroups
}

#[derive(Bundle)]
pub struct GroundTileBundle {
    physical_tile_bundle: PhysicalTileBundle,
    ground_marker: Ground,
    wall_marker: Wall,
}


impl TileBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect, tile_rotation: f32, image_handle: &Handle<Image>) -> Self {
        TileBundle {
            transform: Transform {
                rotation: Quat::from_rotation_z(tile_rotation),
                translation: Vec3::new((grid_pos.x * TILE_SIZE) + TILE_SIZE_HALF, (grid_pos.y * TILE_SIZE) + TILE_SIZE_HALF, 0.0),
                ..default()
            },
            sprite: Sprite {
                image: image_handle.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                rect: Some(atlas_rect),
                ..default()
            },
            stage_marker: StageObject,
        }
        
    }
}

impl PhysicalTileBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect, tile_rotation: f32, image_handle: &Handle<Image>, collision_groups: CollisionGroups) -> Self {
        PhysicalTileBundle {
            tile_bundle: TileBundle::new(stage_creator, grid_pos, atlas_rect, tile_rotation, image_handle),
            rigidbody: RigidBody::Fixed,
            collider: Collider::cuboid(TILE_SIZE_HALF, TILE_SIZE_HALF),
            restitution: Restitution::coefficient(0.0),
            friction: Friction::coefficient(0.0),
            gravity_scale: GravityScale(0.0),
            active_events: ActiveEvents::COLLISION_EVENTS,
            collision_groups
        }
    }
}

impl GroundTileBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect) -> Self {
        GroundTileBundle {
            physical_tile_bundle: PhysicalTileBundle::new(stage_creator, grid_pos, atlas_rect, 0.0, stage_creator.tilemap, CollisionGroups::new(Group::GROUP_1, Group::ALL)),
            ground_marker: Ground,
            wall_marker: Wall,
        }
    }
}