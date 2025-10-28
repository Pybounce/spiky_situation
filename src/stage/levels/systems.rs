
use bevy::prelude::*;

use crate::{databases::game_db::GameDb, game::game_over::GameOver, stage::{levels::data::{CurrentLevelData, LoadLevelEvent, LoadLevelFailedEvent, LoadLevelSuccessEvent}, stage_builder::events::BuildStageEvent}};



pub fn read_level_load_events(
    mut commands: Commands,
    mut load_level_reader: EventReader<LoadLevelEvent>,
    mut level_failed_writer: EventWriter<LoadLevelFailedEvent>,
    mut level_success_writer: EventWriter<LoadLevelSuccessEvent>,
    game_db: Res<GameDb>
) {
    for load_level_event in load_level_reader.read() {
        if let Some(level) = game_db.load_level(load_level_event.level_id) {
            commands.insert_resource(CurrentLevelData {
                spawn_stage_id: level.spawn_stage_id,
                level_id: load_level_event.level_id,
                gateway_pairs: level.gateway_pairs.clone(),
            });
            level_success_writer.write(LoadLevelSuccessEvent { level_id: load_level_event.level_id });
        }
        else {
            level_failed_writer.write(LoadLevelFailedEvent { level_id: load_level_event.level_id });
        }
    }
}

pub fn read_level_load_success_events(
    mut event_reader: EventReader<LoadLevelSuccessEvent>,
    mut stage_build_writer: EventWriter<BuildStageEvent>,
    current_level_data_opt: Option<Res<CurrentLevelData>>,
) {
    let Some(current_level_data) = current_level_data_opt else { return; };

    let mut event_raised = false;
    for _ in event_reader.read() {
        if event_raised == false {
            println!("success");
            stage_build_writer.write(BuildStageEvent {stage_id: current_level_data.spawn_stage_id, gateway_id_opt: None });
            event_raised = true;
        }
    }
}

pub fn read_level_load_failed_events(
    mut event_reader: EventReader<LoadLevelFailedEvent>,
    mut game_over_event_writer: EventWriter<GameOver>,
) {
    for _ in event_reader.read() {
        game_over_event_writer.write(GameOver);
    }
}

