use bevy::prelude::*;

use crate::{common::states::{AppState, GameState}, game::game_over::GameOver};

use super::{StageBuilderData, StageBuilderState};


#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct LoadStageEvent {
    pub stage_id: usize
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct BuildStageEvent {
    pub stage_id: usize
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct StageBuildCompleteEvent {
    pub stage_id: usize
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct StageBuildFailedEvent {
    pub stage_id: usize
}


pub fn read_stage_build_complete_events(
    mut event_reader: EventReader<StageBuildCompleteEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut stage_builder_state: ResMut<NextState<StageBuilderState>>,
) {
    for _ in event_reader.read() {
        game_state.set(GameState::Playing);
        app_state.set(AppState::Game);
        stage_builder_state.set(StageBuilderState::NotBuilding);
    }
}

pub fn read_stage_build_failed_events(
    mut event_reader: EventReader<StageBuildFailedEvent>,
    mut game_over_event_writer: EventWriter<GameOver>
) {
    for _ in event_reader.read() {
        game_over_event_writer.send(GameOver);
    }
}

/// Listens for LoadStageEvent.</br>
/// Begins loading the stage asset.</br>
/// Adds handle to StageBuilderHandles
pub fn read_stage_load_events(
    mut event_reader: EventReader<LoadStageEvent>,
    mut stage_builder_data: ResMut<StageBuilderData>,
    asset_server: Res<AssetServer>,
) {
    for preload_event in event_reader.read() {
        stage_builder_data.stage_id = preload_event.stage_id;
        stage_builder_data.stage_handle = asset_server.load(format!("stages/stage_{}.stage", preload_event.stage_id));
    }
}

/// REQUIRES STAGE LOAD EVENT RAISED </br>
/// Listens for BuildStageEvent. </br>
/// Sets the StageBuilderState to building.
/// (which in turn begins the building of the stage)
pub fn read_stage_build_events(
    mut event_reader: EventReader<BuildStageEvent>,
    mut stage_builder_state: ResMut<NextState<StageBuilderState>>,

) {
    for _ in event_reader.read() {
        stage_builder_state.set(StageBuilderState::Building);
    }
}
