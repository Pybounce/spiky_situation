
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
        .init_asset_loader::<LevelLoader>()
        .add_event::<LoadLevelEvent>()
        .add_event::<LoadLevelFailedEvent>()
        .add_systems(PreUpdate, (read_level_build_events, check_level_asset_loaded));
    }
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