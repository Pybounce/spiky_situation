use bevy::prelude::*;

use crate::stage::stage_objects::StageObject;

use super::{events::{StageBuildCompleteEvent, StageBuildFailedEvent}, stage_asset::Stage, stage_creator::{StageCreator, TILE_SIZE}, CurrentStageData, StageAssets, StageBuilderData};


pub fn unload_old_stage(
    stage_piece_query: Query<(Entity, &StageObject)>,
    mut commands: Commands,
    stage_builder_data: Res<StageBuilderData>,
) {
    for (e, sp) in &stage_piece_query {
        if sp.stage_id != stage_builder_data.stage_id {
            commands.entity(e).despawn();
            commands.remove_resource::<CurrentStageData>();
        }
    }
}

pub fn try_build_stage(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stage_builder_data: Res<StageBuilderData>,
    stage_assets: Res<Assets<Stage>>,
    mut complete_event_writer: EventWriter<StageBuildCompleteEvent>,
    mut failed_event_writer: EventWriter<StageBuildFailedEvent>
) {
    match asset_server.load_state(&stage_builder_data.stage_handle) {
        bevy::asset::LoadState::NotLoaded => {
            failed_event_writer.send(StageBuildFailedEvent { stage_id: stage_builder_data.stage_id });
            return;
        },
        bevy::asset::LoadState::Loading => { return; },
        bevy::asset::LoadState::Loaded => (),
        bevy::asset::LoadState::Failed(_) => {
            failed_event_writer.send(StageBuildFailedEvent { stage_id: stage_builder_data.stage_id });
            return;
        },
    }

    let stage_asset = stage_assets.get(&stage_builder_data.stage_handle);
    let ground_grass_handle: Handle<Image> = asset_server.load("ground_grass.png");
    let ground_snow_handle: Handle<Image> = asset_server.load("ground_snow.png");
    let object_tilemap_handle: Handle<Image> = asset_server.load("object_tilemap.png");

    match stage_asset {
        Some(stage) => {
            let ground_tiles_handle = match stage.terrain_theme {
                super::stage_asset::TerrainTheme::Grass => ground_grass_handle,
                super::stage_asset::TerrainTheme::Snow => ground_snow_handle,
                super::stage_asset::TerrainTheme::Sand => ground_grass_handle,
            };
            commands.insert_resource(StageAssets {
                stage_objects_handle: object_tilemap_handle.clone(),
                ground_tiles_handle: ground_tiles_handle.clone()
            });
            let stage_creator = StageCreator::new(&stage, &ground_tiles_handle, &object_tilemap_handle);
            if stage_creator.build(&mut commands) {
                commands.insert_resource(CurrentStageData {
                    stage_id: stage.id,
                    spawn_translation: (stage.spawn_grid_pos * TILE_SIZE).extend(0.0),
                    bounds: Rect::new(-TILE_SIZE, -TILE_SIZE, stage.grid_width as f32 * TILE_SIZE, stage.grid_height as f32 * TILE_SIZE),
                });
                complete_event_writer.send(StageBuildCompleteEvent { stage_id: stage_builder_data.stage_id });
            }
            else {
                failed_event_writer.send(StageBuildFailedEvent { stage_id: stage_builder_data.stage_id });
            }
        },
        None => {
            failed_event_writer.send(StageBuildFailedEvent { stage_id: stage_builder_data.stage_id });
        },
    }
}
