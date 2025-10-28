
use bevy::prelude::*;

use crate::stage::levels::data::*;
use crate::stage::levels::systems::*;

pub mod data;
pub mod systems;

pub struct LevelBuilderPlugin;

impl Plugin for LevelBuilderPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<LoadLevelEvent>()
        .add_event::<LoadLevelFailedEvent>()
        .add_event::<LoadLevelSuccessEvent>()
        .add_systems(Update, (read_level_load_events, read_level_load_failed_events, read_level_load_success_events));
    }
}

