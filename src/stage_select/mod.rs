use bevy::prelude::*;

use crate::{common::states::{AppState, DespawnOnStateExit}, stage::stage_builder::events::{BuildStageEvent, LoadStageEvent}, stage_editor::StageEditorLoadDetails};


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
        text: Text::from_section("Stage Select", TextStyle::default()),
        ..default()
    })
    .insert(DespawnOnStateExit::App(AppState::MainMenu));
}

pub fn try_enter_stage(
    input: Res<ButtonInput<KeyCode>>,
    mut load_event_writer: EventWriter<LoadStageEvent>,
    mut build_event_writer: EventWriter<BuildStageEvent>
) {
    if input.just_released(KeyCode::Space) {
        load_event_writer.send(LoadStageEvent {stage_id: 0});
        build_event_writer.send(BuildStageEvent {stage_id: 0});
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
