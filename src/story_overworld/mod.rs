
use bevy::prelude::*;

use crate::{common::states::AppState, databases::game_db::GameDb, stage::levels::data::LoadLevelEvent, story_overworld::ui::build_story_overworld};

mod ui;

pub struct StoryOverworldPlugin;

impl Plugin for StoryOverworldPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::StoryOverworld), build_story_overworld)
        .add_systems(Update, (check_play_level_button).run_if(in_state(AppState::StoryOverworld)));
    }
}

#[derive(Component)]
pub struct PlayLevelButton(usize);



fn check_play_level_button(
    mut interaction_query: Query<(&PlayLevelButton, &Interaction), (Changed<Interaction>, With<Button>)>,
    mut load_level_event_writer: EventWriter<LoadLevelEvent>,
) {
    for (play_level_btn, interaction) in &mut interaction_query {

        match interaction {
            Interaction::Pressed => { 
                load_level_event_writer.write(LoadLevelEvent { level_id: play_level_btn.0 });
            },
            _ => ()
        }
    }
}