
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{databases::save_db::SaveGame, game::endless::components::EndlessRun, local_player::LocalPlayer, stage::{stage_builder::{events::{BuildStageEvent, LoadStageEvent}, CurrentStageData}, stage_objects::goal::StageGoal}};

#[derive(Event)]
pub struct GoalReached {
    pub stage_id: usize
}


pub fn check_goal_reached(
    player_query: Query<&CollidingEntities, With<LocalPlayer>>,
    goal_query: Query<(), With<StageGoal>>,
    mut event_writer: EventWriter<GoalReached>,
    stage_data_opt: Option<Res<CurrentStageData>>,
) {
    if let Some(stage_data) = stage_data_opt {
        for colliding_entities in &player_query {
            for colliding_entity in colliding_entities.iter() {
                if let Ok(_) = goal_query.get(colliding_entity) {
                    event_writer.send(GoalReached { stage_id: stage_data.stage_id });
                }
            }
        }
    }

}

pub fn next_staged_if_goal_reached(
    stage_data_opt: Option<Res<CurrentStageData>>,
    mut load_event_writer: EventWriter<LoadStageEvent>,
    mut build_event_writer: EventWriter<BuildStageEvent>,
    mut event_reader: EventReader<GoalReached>,
    mut current_run: ResMut<EndlessRun>,
    mut save_writer: EventWriter<SaveGame>,
) {
    if let Some(stage_data) = stage_data_opt {
        let mut build_event_raised = false;
        for event in event_reader.read() {
            if event.stage_id == stage_data.stage_id && !build_event_raised {
                save_writer.send(SaveGame);
                current_run.complete_stage();
                load_event_writer.send(LoadStageEvent {stage_id: current_run.current_stage_id() });
                build_event_writer.send(BuildStageEvent {stage_id: current_run.current_stage_id() });
                build_event_raised = true;
            }
        }
    }
}