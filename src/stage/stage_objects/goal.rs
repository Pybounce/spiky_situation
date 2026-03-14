use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{common::{bloom::Bloomin, physics::layers::GamePhysicsLayer}, rt_lights::components::PointLight, stage::stage_builder::stage_creator::StageCreator};

use super::tiles::PhysicalTileBundle;


#[derive(Component)]
pub struct StageGoal;


pub struct GoalFactory;

impl GoalFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, grid_pos: Vec2, atlas_rect: Rect) {
        commands.spawn((
            PhysicalTileBundle::new(stage_creator, grid_pos, atlas_rect, 0.0, stage_creator.object_tilemap, stage_creator.object_specular_tilemap.into(), CollisionLayers::new(GamePhysicsLayer::StageObject, LayerMask::ALL)),
            StageGoal,
            PointLight {
                intensity: 1.0,
                colour: Color::srgb_u8(0, 255, 255),
            },
            Bloomin(3.0)
        ));
    }
}