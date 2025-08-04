
use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionGroups, Group, RigidBody};

use crate::{common::{animated_sprite::{AnimateOnTouch, SpriteAnimator}, physics::bouncy::Bouncy}, ground::Ground, local_player::FORCE_MUL, stage::stage_builder::stage_creator::{StageCreator, TILE_SIZE_HALF}};

use super::{tiles::TileBundle, StageObject};

const SPRING_BOUNCE_FORCE: f32 = 2000.0 * FORCE_MUL;

#[derive(Component)]
pub struct Spring;

pub struct SpringFactory;

impl SpringFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, grid_pos: Vec2, atlas_rects: Vec<Rect>, rotation: f32) {

        commands.spawn((
            TileBundle::new(stage_creator, grid_pos, atlas_rects[0], rotation, stage_creator.object_tilemap),
            SpriteAnimator::new_non_repeating(50, atlas_rects),
        )).with_children(|parent| {
            parent.spawn((
                Collider::cuboid(TILE_SIZE_HALF * 0.9, TILE_SIZE_HALF * 0.2),
                Transform::from_xyz(0.0, 0.0, 0.0),
                CollisionGroups::new(Group::GROUP_3, Group::ALL),
                ActiveEvents::COLLISION_EVENTS,
                RigidBody::Fixed,
                Spring,
                Bouncy {
                    force: Vec2::from_angle(rotation + (PI / 2.0)) * SPRING_BOUNCE_FORCE,
                },
                StageObject,
                AnimateOnTouch {
                    animator_entity: parent.target_entity(),
                }
            ));
            parent.spawn((
                Collider::cuboid(TILE_SIZE_HALF, TILE_SIZE_HALF * 0.4),
                Transform::from_xyz(0.0, -TILE_SIZE_HALF, 0.0),
                CollisionGroups::new(Group::GROUP_1, Group::ALL),
                ActiveEvents::COLLISION_EVENTS,
                RigidBody::Fixed,
                Ground,
                StageObject,
            ));
        });

    }
}

