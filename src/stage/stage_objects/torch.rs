use bevy::prelude::*;
use bevy_seedling::{firewheel::dsp::distance_attenuation::DistanceAttenuation, prelude::SpatialBasicNode, sample::SamplePlayer, sample_effects, spatial::SpatialScale};

use crate::{common::{animated_sprite::SpriteAnimator, bloom::Bloomin}, rt_lights::components::PointLight, stage::{stage_builder::{stage_asset, stage_creator::StageCreator}, stage_objects::tiles::TileBundle}};


pub struct TorchFactory;

impl TorchFactory {
    pub fn spawn(commands: &mut Commands, asset_server: &AssetServer, stage_creator: &StageCreator, atlas_rects: Vec<Rect>, torch: &stage_asset::Torch) {
        commands.spawn((
            TileBundle::new(stage_creator, torch.grid_pos, atlas_rects[0], 0.0, stage_creator.object_tilemap, stage_creator.object_specular_tilemap.into()),
            SpriteAnimator::new(200, atlas_rects),
            PointLight {
                intensity: 1.0,
                colour: Color::srgb_u8(255, 176, 55),
            },
            Bloomin(1.0),
            SamplePlayer::new(asset_server.load("audio/sfx/fire_crackling.wav")).looping(),
            sample_effects![SpatialBasicNode {
                distance_attenuation: DistanceAttenuation {
                    reference_distance: 32.0, 
                    max_distance: 160.0, 
                    max_muffle_distance: 160.0, 
                    
                    ..default()
                },
                ..default()
            }],
        ));
    }
}