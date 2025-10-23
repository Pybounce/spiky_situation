use bevy::prelude::*;

use crate::{common::{animated_sprite::SpriteAnimator, bloom::Bloomin}, rt_lights::components::PointLight, stage::{stage_builder::{stage_asset, stage_creator::StageCreator}, stage_objects::tiles::TileBundle}};


pub struct TorchFactory;

impl TorchFactory {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, atlas_rects: Vec<Rect>, torch: &stage_asset::Torch) {
        commands.spawn((
            TileBundle::new(stage_creator, torch.grid_pos, atlas_rects[0], 0.0, stage_creator.object_tilemap),
            SpriteAnimator::new(200, atlas_rects),
            PointLight {
                intensity: 1.0,
                colour: Color::srgb_u8(255, 176, 55),
            },
            Bloomin(1.0)
        ));
    }
}