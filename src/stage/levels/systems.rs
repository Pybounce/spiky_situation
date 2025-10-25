
use bevy::prelude::*;

use crate::{game::game_over::GameOver, stage::{levels::{data::{CurrentLevelData, LevelBuilderData, LoadLevelEvent, LoadLevelFailedEvent, LoadLevelSuccessEvent}, level_asset::Level, LevelLoaderState}, stage_builder::events::BuildStageEvent}};



pub fn read_level_load_events(
    mut commands: Commands,
    mut load_level_reader: EventReader<LoadLevelEvent>,
    asset_server: Res<AssetServer>,
    mut level_load_state: ResMut<NextState<LevelLoaderState>>,
) {
    for load_level_event in load_level_reader.read() {
        commands.insert_resource(LevelBuilderData {
            level_id: load_level_event.level_id,
            level_handle: asset_server.load(format!("levels/level_{}.level", load_level_event.level_id)),
        });
        level_load_state.set(LevelLoaderState::Loading);
    }
}

pub fn read_level_load_success_events(
    mut event_reader: EventReader<LoadLevelSuccessEvent>,
    mut stage_build_writer: EventWriter<BuildStageEvent>,
    current_level_data_opt: Option<Res<CurrentLevelData>>,
    mut level_load_state: ResMut<NextState<LevelLoaderState>>,
) {
    let Some(current_level_data) = current_level_data_opt else { return; };

    let mut event_raised = false;
    for _ in event_reader.read() {
        if event_raised == false {
            level_load_state.set(LevelLoaderState::NotLoading);
            println!("success");
            stage_build_writer.write(BuildStageEvent {stage_id: current_level_data.spawn_stage_id, gateway_id_opt: None });
            event_raised = true;
        }
    }
}

pub fn read_level_load_failed_events(
    mut event_reader: EventReader<LoadLevelFailedEvent>,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut level_load_state: ResMut<NextState<LevelLoaderState>>,
) {
    for _ in event_reader.read() {
        level_load_state.set(LevelLoaderState::NotLoading);
        game_over_event_writer.write(GameOver);
    }
}

pub fn try_load_level(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    level_data_opt: Option<Res<LevelBuilderData>>,
    mut level_failed_writer: EventWriter<LoadLevelFailedEvent>,
    mut level_success_writer: EventWriter<LoadLevelSuccessEvent>,
    level_assets: Res<Assets<Level>>,
) {
    let Some(level_data) = level_data_opt else { return; };


    match asset_server.load_state(&level_data.level_handle) {
        bevy::asset::LoadState::NotLoaded => {
            level_failed_writer.write(LoadLevelFailedEvent { level_id: level_data.level_id });
            return;
        },
        bevy::asset::LoadState::Loading => { return; },
        bevy::asset::LoadState::Loaded => {

            match level_assets.get(&level_data.level_handle) {
                Some(level_asset) => {
                    commands.insert_resource(CurrentLevelData {
                        spawn_stage_id: level_asset.spawn_stage_id,
                        level_id: level_data.level_id,
                        gateway_pairs: level_asset.gateway_pairs.clone(),
                    });
                    level_success_writer.write(LoadLevelSuccessEvent { level_id: level_data.level_id });
                },
                None => {
                    level_failed_writer.write(LoadLevelFailedEvent { level_id: level_data.level_id });
                    return;
                },
            }
        },
        bevy::asset::LoadState::Failed(_) => {
            level_failed_writer.write(LoadLevelFailedEvent { level_id: level_data.level_id });
            return;
        },
    }
}



