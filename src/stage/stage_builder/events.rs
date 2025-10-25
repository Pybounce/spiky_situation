use bevy::prelude::*;

use crate::{common::states::{AppState, GameState}, game::game_over::GameOver};

use super::{StageBuilderData, StageBuilderState};


// Future TODO fix
// To preload a stage, just load in the Stage Handle and store in a resource so it's not unloaded
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
//pub struct LoadStageEvent {
//    pub stage_id: usize
//}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct BuildStageEvent {
    pub stage_id: usize,
    pub gateway_id_opt: Option<usize>
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
    mut game_over_event_writer: EventWriter<GameOver>,
    mut stage_builder_state: ResMut<NextState<StageBuilderState>>,
) {
    for _ in event_reader.read() {
        stage_builder_state.set(StageBuilderState::NotBuilding);
        game_over_event_writer.write(GameOver);
    }
}


/// REQUIRES STAGE LOAD EVENT RAISED </br>
/// Listens for BuildStageEvent. </br>
/// Sets the StageBuilderState to building.
/// (which in turn begins the building of the stage)
pub fn read_stage_build_events(
    mut event_reader: EventReader<BuildStageEvent>,
    mut stage_builder_state: ResMut<NextState<StageBuilderState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,

) {
    for build_stage_event in event_reader.read() {
        commands.insert_resource(StageBuilderData {
            stage_id: build_stage_event.stage_id,
            stage_handle: asset_server.load(format!("stages/stage_{}.stage", build_stage_event.stage_id)),
            gateway_id_opt: build_stage_event.gateway_id_opt
        });
        stage_builder_state.set(StageBuilderState::Building);
    }
}
