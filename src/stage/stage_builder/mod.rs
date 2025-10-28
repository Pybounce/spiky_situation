use bevy::prelude::*;
use events::{read_stage_build_complete_events, read_stage_build_events, read_stage_build_failed_events, BuildStageEvent, StageBuildCompleteEvent, StageBuildFailedEvent};
use systems::unload_old_stage;

use crate::common::states::AppState;

pub mod events;
pub mod stage_asset;
mod systems;
pub mod stage_creator;

pub struct StageBuilderPlugin;

impl Plugin for StageBuilderPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<BuildStageEvent>()
        .add_event::<StageBuildCompleteEvent>()
        .add_event::<StageBuildFailedEvent>()
        .add_systems(PreUpdate, (unload_old_stage, read_stage_build_events).chain())
        .add_systems(Update, (read_stage_build_complete_events, read_stage_build_failed_events))
        .add_systems(OnExit(AppState::Game), unload_old_stage);
    }
}


#[derive(Resource, Default)]
pub struct CurrentStageData {
    pub stage_id: usize,
    pub gateway_id_opt: Option<usize>,
    pub bounds: Rect
}

#[derive(Resource, Default)]
pub struct StageAssets {
    pub stage_objects_handle: Handle<Image>,
    pub ground_tiles_handle: Handle<Image>,
}