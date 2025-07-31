
use bevy::prelude::*;

use crate::{common::states::{AppState, GameState}, game::current_run::CurrentRun, stage::stage_builder::StageBuilderState};


#[derive(Event)]
pub struct GameOver;



pub fn read_game_over_events(
    mut event_reader: EventReader<GameOver>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut stage_builder_state: ResMut<NextState<StageBuilderState>>,
    mut commands: Commands
) {
    for _ in event_reader.read() {
        game_state.set(GameState::NA);
        app_state.set(AppState::MainMenu);
        stage_builder_state.set(StageBuilderState::NotBuilding);
        commands.remove_resource::<CurrentRun>();
    }
}
