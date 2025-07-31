
use bevy::prelude::*;

use crate::common::states::{AppState, DespawnOnStateExit};

#[derive(Resource)]
pub struct CurrentRun {
    lives_remaining: u32,
    stages_complete: u32,
    stage_ids: Vec<usize>,
    stage_index: usize,
}


impl CurrentRun {
    pub fn new(stage_ids: Vec<usize>, lives: u32) -> Self {
        //TODO: Shuffle stage ids here
        Self {
            lives_remaining: lives,
            stages_complete: 0,
            stage_ids,
            stage_index: 0,
        }
    }
    pub fn complete_stage(&mut self) {
        self.stages_complete += 1;
        //TODO: If resettings, shuffle stage ids here
        self.stage_index = (self.stage_index  + 1) % self.stage_ids.len();
    }
    pub fn current_stage_id(&self) -> usize {
        return self.stage_ids[self.stage_index];
    }
    pub fn lives_remaining(&self) -> u32 {
        return self.lives_remaining;
    }
    pub fn remove_life(&mut self) {
        self.lives_remaining = self.lives_remaining.saturating_sub(1);
    }
}

#[derive(Component)]
pub struct LivesRemainingText;

#[derive(Component)]
pub struct StagesCompleteText;

pub fn update_lives_remaining_text(
    mut query: Query<&mut Text, With<LivesRemainingText>>,
    current_run_opt: Option<Res<CurrentRun>>
) {
    if let Some(current_run) = current_run_opt {
        for mut text in &mut query {
            text.sections[0].value = current_run.lives_remaining().to_string();
        }
    }
}

pub fn update_stages_complete(
    mut query: Query<&mut Text, With<StagesCompleteText>>,
    current_run_opt: Option<Res<CurrentRun>>
) {
    if let Some(current_run) = current_run_opt {
        for mut text in &mut query {
            text.sections[0].value = current_run.stages_complete.to_string();
        }
    }
}

pub fn add_current_run_ui(
    mut commands: Commands
) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("-", TextStyle::default()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0)),
            ..default()
        },
        LivesRemainingText
    ))
    .insert(DespawnOnStateExit::App(AppState::Game));
    
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("-", TextStyle::default()),
            ..default()
        },
        StagesCompleteText
    ))
    .insert(DespawnOnStateExit::App(AppState::Game));
}