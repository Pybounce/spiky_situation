use bevy::prelude::*;
use stage_goal::{check_goal_reached, next_staged_if_goal_reached, GoalReached};

use crate::{common::states::{AppState, GameState}, game::{current_run::{add_current_run_ui, update_lives_remaining_text, update_stages_complete}, game_over::{read_game_over_events, GameOver}}};

pub mod stage_goal;
pub mod current_run;
pub mod game_over;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<GoalReached>()
        .add_event::<GameOver>()
        .add_systems(OnEnter(AppState::Game), add_current_run_ui)
        .add_systems(Update, (read_game_over_events, check_goal_reached, next_staged_if_goal_reached).run_if(in_state(AppState::Game)).run_if(in_state(GameState::Playing)))
        .add_systems(Update, (update_lives_remaining_text, update_stages_complete));
    }
}


