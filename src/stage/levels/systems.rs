
use bevy::prelude::*;

use crate::stage::levels::data::{LevelBuilderData, LoadLevelEvent, LoadLevelFailedEvent};



pub fn read_level_build_events(
    mut commands: Commands,
    mut build_level_reader: EventReader<LoadLevelEvent>,
    asset_server: Res<AssetServer>,
) {
    for build_level_event in build_level_reader.read() {
        commands.insert_resource(LevelBuilderData {
            level_id: build_level_event.level_id,
            level_handle: asset_server.load(format!("levels/level_{}.level", build_level_event.level_id)),
        });
    }
}

pub fn check_level_asset_loaded(
    asset_server: Res<AssetServer>,
    level_data_opt: Option<Res<LevelBuilderData>>,
    mut level_failed_writer: EventWriter<LoadLevelFailedEvent>,
) {
    let Some(level_data) = level_data_opt else { return; };


    match asset_server.load_state(&level_data.level_handle) {
        bevy::asset::LoadState::NotLoaded => {
            level_failed_writer.write(LoadLevelFailedEvent { level_id: level_data.level_id });
            return;
        },
        bevy::asset::LoadState::Loading => { return; },
        bevy::asset::LoadState::Loaded => (),
        bevy::asset::LoadState::Failed(_) => {
            level_failed_writer.write(LoadLevelFailedEvent { level_id: level_data.level_id });
            return;
        },
    }
}



