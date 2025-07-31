use std::{fs, path::Path};

use bevy::{prelude::*, scene::ron::from_str};

use crate::{common::states::{AppState, DespawnOnStateExit}, game::endless::components::EndlessRun, stage::stage_builder::events::{BuildStageEvent, LoadStageEvent}, stage_editor::StageEditorLoadDetails};


pub struct StageSelectPlugin;

impl Plugin for StageSelectPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::MainMenu), display_stage_select)
        .add_systems(Update, (try_enter_stage, try_enter_stage_editor).run_if(in_state(AppState::MainMenu)));
    }
}




pub fn display_stage_select(
    mut commands: Commands
) {
    commands.spawn(Text2dBundle {
        text: Text::from_section("Press some shit to start a new run", TextStyle::default()),
        ..default()
    })
    .insert(DespawnOnStateExit::App(AppState::MainMenu));
}

pub fn try_enter_stage(
    input: Res<ButtonInput<KeyCode>>,
    mut load_event_writer: EventWriter<LoadStageEvent>,
    mut build_event_writer: EventWriter<BuildStageEvent>,
    mut commands: Commands
) {
    if input.just_released(KeyCode::Space) {
        let stage_ids = get_stage_ids();
        let current_run = EndlessRun::new(stage_ids, 10);

        load_event_writer.send(LoadStageEvent {stage_id: current_run.current_stage_id() });
        build_event_writer.send(BuildStageEvent {stage_id: current_run.current_stage_id() });

        commands.insert_resource(current_run);
    }
}

pub fn try_enter_stage_editor(
    input: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut commands: Commands
) {
    if input.just_released(KeyCode::KeyE) {
        commands.insert_resource(StageEditorLoadDetails {
            template_stage_id: 5.into(),
            new_stage_id: 5,
            template_stage_handle: None
        });
        app_state.set(AppState::StageEditor);
    }
}

/// Returns a vector of stageIds that match the stage query for the new run (order does not matter)
fn get_stage_ids() -> Vec<usize> {
    let path = "assets/stages";
    let dir = Path::new(path);
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