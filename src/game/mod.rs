use bevy::prelude::*;
use stage_goal::{check_goal_reached, next_staged_if_goal_reached, GoalReached};

use crate::{common::states::{AppState, GameMode, GameState}, game::{endless::{systems::{check_death_endless_mode, save_endless_game}, ui::{add_endless_mode_ui, update_endless_lives_remaining_text, update_endless_stages_complete}}, game_over::{read_game_over_events, GameOver}, stage_goal::skip_stage}};

pub mod stage_goal;
pub mod game_over;
pub mod endless;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<GoalReached>()
        .add_event::<GameOver>()
        .add_systems(OnEnter(GameMode::Endless), add_endless_mode_ui)
        .add_systems(Update, (read_game_over_events, check_goal_reached, skip_stage, next_staged_if_goal_reached).run_if(in_state(AppState::Game)).run_if(in_state(GameState::Playing)))
        .add_systems(Update, (save_endless_game, check_death_endless_mode, update_endless_lives_remaining_text, update_endless_stages_complete).run_if(in_state(GameMode::Endless)));
    }
}
