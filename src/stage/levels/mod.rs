
use bevy::prelude::*;

use crate::stage::levels::data::*;
use crate::stage::levels::level_asset::*;
use crate::stage::levels::systems::*;

pub mod level_asset;
pub mod data;
pub mod systems;

pub struct LevelBuilderPlugin;

impl Plugin for LevelBuilderPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_asset::<Level>()
        .init_asset_loader::<LevelLoader>()
        .init_state::<LevelLoaderState>()
        .add_event::<LoadLevelEvent>()
        .add_event::<LoadLevelFailedEvent>()
        .add_event::<LoadLevelSuccessEvent>()
        .add_systems(Update, (try_load_level).run_if(in_state(LevelLoaderState::Loading)))
        .add_systems(Update, (read_level_load_events, read_level_load_failed_events, read_level_load_success_events));
    }
}


#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum LevelLoaderState {
    #[default]
    NotLoading,
    Loading,
}

//  Currently...
//      I can raise level load events
//      Reading these events triggers the loading of the level asset

//  To do...
//      Once loaded successfully, raise a stagebuild/load event for the spawn room
//      Add in an editor object for the gateways
//      Get in some logic to move between them nicely

//  To do after...
//      Make an actual save file that has completed levels? (how does this work with custom not sure)
//      Collecting the goal in a stage should trigger a LEVEL complete, not stage complete
//      Remove goals from stages