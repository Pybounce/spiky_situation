
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
//      I can raise level load events and load the level and first stage
//      I can setup gateway links in the level file

//  To do
//      Make an actual save file that has completed levels? (how does this work with custom not sure) - can just have a hash for completed levels and check myhash.contains(level) etc
//      Collecting the goal in any stage should trigger a LEVEL complete, not stage complete
//      Remove goals from stages
//      Theory craft how things like lock blocks (or dynamic blocks in general) will work

//  To do after...
//      Ability to have vertical gateways
//      Screen transition for gateways so it's less jarring
//      Rework the gamemode bs
//          An endless is still good if I add a random level generator using rooms