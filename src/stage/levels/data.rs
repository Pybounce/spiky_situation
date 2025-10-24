
use bevy::prelude::*;

use crate::stage::levels::level_asset::Level;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct LoadLevelEvent {
    pub level_id: usize
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct LoadLevelFailedEvent {
    pub level_id: usize
}

#[derive(Resource)]
pub struct LevelBuilderData {
    pub level_id: usize,
    pub level_handle: Handle<Level>
}