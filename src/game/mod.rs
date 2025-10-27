use bevy::prelude::*;
use stage_goal::{check_goal_reached, GoalReached};

use crate::{common::states::{AppState, GameMode, GameState}, game::{game_over::{read_game_over_events, GameOver}, stage_goal::skip_stage}};

pub mod stage_goal;
pub mod game_over;
pub mod story;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<GoalReached>()
        .add_event::<GameOver>()
        .add_systems(Update, (read_game_over_events, check_goal_reached, skip_stage).run_if(in_state(AppState::Game)).run_if(in_state(GameState::Playing)));
    }
}
