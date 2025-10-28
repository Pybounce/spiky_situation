
use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{common::{physics::avian_ex::ManyCollidingEntities, states::{AppState, GameState}}, databases::{game_db::GameDb, save_db::{SaveDb, SaveGame}}, game::story::StorySave, local_player::LocalPlayer, stage::{levels::data::CurrentLevelData, stage_builder::CurrentStageData, stage_objects::goal::StageGoal}};

#[derive(Event)]
pub struct GoalReached {
    pub stage_id: usize
}


pub fn check_goal_reached(
    player_query: Query<&ManyCollidingEntities, With<LocalPlayer>>,
    goal_query: Query<(), With<StageGoal>>,
    mut event_writer: EventWriter<GoalReached>,
    stage_data_opt: Option<Res<CurrentStageData>>,
) {
    if let Some(stage_data) = stage_data_opt {
        for colliding_entities in &player_query {
            for colliding_entity in colliding_entities.iter() {
                if let Ok(_) = goal_query.get(*colliding_entity) {
                    event_writer.write(GoalReached { stage_id: stage_data.stage_id });
                }
            }
        }
    }

}


pub fn skip_stage(
    mut event_writer: EventWriter<GoalReached>,
    stage_data_opt: Option<Res<CurrentStageData>>,
    input: Res<ButtonInput<KeyCode>>,
    gamepad_query: Query<&Gamepad>
) {
    let Some(stage_data) = stage_data_opt else { return; };
    if input.just_pressed(KeyCode::KeyN) {
        event_writer.write(GoalReached { stage_id: stage_data.stage_id });
    }
    for gamepad in &gamepad_query {
        if gamepad.just_pressed(GamepadButton::LeftTrigger) {
            event_writer.write(GoalReached { stage_id: stage_data.stage_id });
        }
    }
}


pub fn story_save_goal_reached(
    story_save_opt: Option<ResMut<StorySave>>,
    mut event_reader: EventReader<GoalReached>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
    game_db: Res<GameDb>,
    current_level_data_opt: Option<Res<CurrentLevelData>>

) {
    let Some(mut story_save) = story_save_opt else { return; };
    let Some(current_level_data) = current_level_data_opt else { return; };

    if event_reader.read().count() > 0 {
        story_save.completed_levels.insert(current_level_data.level_id);
        game_db.save_story(&story_save);
        game_state.set(GameState::NA);
        app_state.set(AppState::StoryOverworld);
    }

}