
use std::f32::consts::PI;

use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{common::{animated_sprite::{AnimateOnTouch, SpriteAnimator}, physics::{bouncy::Bouncy, layers::GamePhysicsLayer}}, ground::Ground, local_player::FORCE_MUL, stage::stage_builder::stage_creator::{StageCreator, TILE_SIZE, TILE_SIZE_HALF}};

use super::{tiles::TileBundle, StageObject};

const SPRING_BOUNCE_FORCE: f32 = 2000.0 * FORCE_MUL;

#[derive(Component)]
pub struct Spring;

pub struct SpringFactory;

impl SpringFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, grid_pos: Vec2, atlas_rects: Vec<Rect>, rotation: f32) {
        let mut mask = LayerMask(GamePhysicsLayer::StageObject.to_bits());
        mask.add(GamePhysicsLayer::Player.to_bits());

        commands.spawn((
            TileBundle::new(stage_creator, grid_pos, atlas_rects[0], rotation, stage_creator.object_tilemap),
            SpriteAnimator::new_non_repeating(50, atlas_rects),
            RigidBody::Static,
            Spring,
            Bouncy {
                force: Vec2::from_angle(rotation + (PI / 2.0)) * SPRING_BOUNCE_FORCE,
            },
            StageObject::Volatile,
            AnimateOnTouch {
                    animator_entity: None,
            },
            children![(
                Collider::rectangle(TILE_SIZE * 0.9, TILE_SIZE * 0.3),
                Transform::from_xyz(0.0, -TILE_SIZE_HALF / 3.0, 0.0),
                CollisionEventsEnabled,
                CollisionLayers::new(GamePhysicsLayer::StageObject, mask)
            )]
        ));

    }
}

