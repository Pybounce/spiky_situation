
use bevy::prelude::*;

#[derive(Resource)]
pub struct CurrentRun {
    lives_remaining: u32,
    stages_complete: u32,
    stage_ids: Vec<usize>,
    stage_index: usize
}


impl CurrentRun {
    pub fn new(stage_ids: Vec<usize>, lives: u32) -> Self {
        //TODO: Shuffle stage ids here
        Self {
            lives_remaining: lives,
            stages_complete: 0,
            stage_ids,
            stage_index: 0,
        }
    }
    pub fn complete_stage(&mut self) {
        self.stages_complete += 1;
        //TODO: If resettings, shuffle stage ids here
        self.stage_index = (self.stage_index  + 1) % self.stage_ids.len();
    }
    pub fn current_stage_id(&self) -> usize {
        return self.stage_ids[self.stage_index];
    }
}