use std::{env, fs, path::PathBuf};

use bevy::prelude::*;

use crate::{common::states::AppState, game::story::StorySave, main_menu::ui::{build_main_menu_ui, check_continue_game_interaction, check_new_game_interaction, check_new_game_interaction_TEMP_GAMEPAD_SUPPORT}, stage::{levels::data::LoadLevelEvent, stage_builder::events::BuildStageEvent}, stage_editor::StageEditorLoadDetails};

pub mod ui;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<LoadSave>()
        .add_systems(OnEnter(AppState::MainMenu), build_main_menu_ui)
        .add_systems(Update, (try_start_game, check_new_game_interaction_TEMP_GAMEPAD_SUPPORT, check_new_game_interaction, check_continue_game_interaction, try_enter_stage_editor).run_if(in_state(AppState::MainMenu)));
    }
}


#[derive(Event)]
pub enum LoadSave {
    Story(StorySave)
}

pub fn try_start_game(
    mut start_game_reader: EventReader<LoadSave>,
    //mut load_level_event_writer: EventWriter<LoadLevelEvent>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let mut game_started = false;
    for event in start_game_reader.read() {
        if game_started { continue; }
        match event {
            LoadSave::Story(story_save) => {
                //load_level_event_writer.write(LoadLevelEvent { level_id: 0 });
                commands.insert_resource(story_save.clone());
                game_started = true;
                app_state.set(AppState::StoryOverworld);
            },
        }
    }
}

pub fn try_enter_stage_editor(
    input: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut commands: Commands
) {
    if input.just_released(KeyCode::KeyE) {
        commands.insert_resource(StageEditorLoadDetails {
            template_stage_id: 8.into(),
            new_stage_id: 8,
        });
        app_state.set(AppState::StageEditor);
    }
}