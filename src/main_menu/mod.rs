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
    //mut build_event_writer: EventWriter<BuildStageEvent>,
    mut load_level_event_writer: EventWriter<LoadLevelEvent>,
    mut commands: Commands
) {
    let mut game_started = false;
    if start_game_reader.read().count() > 0 {}
    for event in start_game_reader.read() {
        if game_started { continue; }
        match event {
            LoadSave::Story(story_save) => {
                //load_level_event_writer.write(LoadLevelEvent { level_id: 0 });
                commands.insert_resource(story_save.clone());
                game_started = true;
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
            template_stage_id: 1.into(),
            new_stage_id: 1,
            template_stage_handle: None
        });
        app_state.set(AppState::StageEditor);
    }
}

/// Returns a vector of stageIds that match the stage query for the new run (order does not matter)
fn get_stage_ids() -> Vec<usize> {

    let dir = if let Ok(manifest_dir) = env::var("BEVY_ASSET_ROOT") {
        PathBuf::from(manifest_dir)
    } else if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(manifest_dir)
    } else {
        env::current_exe()
            .map(|path| path.parent().map(ToOwned::to_owned).unwrap())
            .unwrap()
    }.join("assets").join("stages");

    let mut stages = Vec::<usize>::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    let id_string = name.strip_prefix("stage_").unwrap().strip_suffix(".stage").unwrap();
                    if let Ok(id) = id_string.parse::<usize>() {
                        stages.push(id);
                    }
                }
            }
        }
    }
    return stages;
}