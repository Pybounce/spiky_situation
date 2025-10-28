
use bevy::prelude::*;

use crate::common::pair_map::PairMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct LoadLevelEvent {
    pub level_id: usize
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct LoadLevelFailedEvent {
    pub level_id: usize
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Event)]
pub struct LoadLevelSuccessEvent {
    pub level_id: usize
}


#[derive(Resource)]
pub struct CurrentLevelData {
    pub level_id: usize,
    pub spawn_stage_id: usize,
    /// (stageId, gatewayId)
    pub gateway_pairs: PairMap<(usize, usize)>
}