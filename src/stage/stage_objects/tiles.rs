
use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{common::physics::layers::GamePhysicsLayer, ground::Ground, lit_sprite::global_components::LitSprite, rt_lights::components::{LightOccluder, StaticLightOccluder}, stage::{stage_builder::stage_creator::{StageCreator, TILE_SIZE, TILE_SIZE_HALF}, stage_objects::StageObject}, wall::Wall};



#[derive(Bundle)]
pub struct TileBundle {
    pub sprite: LitSprite,
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
    pub collision_events_enabled: CollisionEventsEnabled,
    pub collision_layers: CollisionLayers
}

#[derive(Bundle)]
pub struct GroundTileBundle {
    physical_tile_bundle: PhysicalTileBundle,
    ground_marker: Ground,
    wall_marker: Wall,
    occluder: LightOccluder,
    static_occluder: StaticLightOccluder
}


impl TileBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect, tile_rotation: f32, image_handle: &Handle<Image>) -> Self {
        TileBundle {
            transform: Transform {
                rotation: Quat::from_rotation_z(tile_rotation),
                translation: Vec3::new((grid_pos.x * TILE_SIZE) + TILE_SIZE_HALF, (grid_pos.y * TILE_SIZE) + TILE_SIZE_HALF, 0.0),
                ..default()
            },
            sprite: LitSprite {
                albedo_texture: image_handle.clone().into(),
                size: Vec2::new(TILE_SIZE, TILE_SIZE),
                rect: Some(atlas_rect),
                ..default()
            },
            stage_marker: StageObject::Volatile,
        }
        
    }
}

impl PhysicalTileBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect, tile_rotation: f32, image_handle: &Handle<Image>, collision_layers: CollisionLayers) -> Self {
        PhysicalTileBundle {
            tile_bundle: TileBundle::new(stage_creator, grid_pos, atlas_rect, tile_rotation, image_handle),
            rigidbody: RigidBody::Static,
            collider: Collider::rectangle(TILE_SIZE, TILE_SIZE),
            restitution: Restitution::new(0.0),
            friction: Friction::new(0.0),
            gravity_scale: GravityScale(0.0),
            collision_events_enabled: CollisionEventsEnabled,
            collision_layers
        }
    }
}

impl GroundTileBundle {
    pub fn new(stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect) -> Self {
        GroundTileBundle {
            physical_tile_bundle: PhysicalTileBundle::new(stage_creator, grid_pos, atlas_rect, 0.0, stage_creator.tilemap, CollisionLayers::new(GamePhysicsLayer::Ground, LayerMask::ALL)),
            ground_marker: Ground,
            wall_marker: Wall,
            occluder: LightOccluder::Rect(16.0, 16.0),
            static_occluder: StaticLightOccluder
        }
    }
}