

use bevy::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct EndlessRun {
    lives_remaining: u32,
    stages_complete: u32,
    stage_ids: Vec<usize>,
    stage_index: usize,
}


impl EndlessRun {
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
    pub fn lives_remaining(&self) -> u32 {
        return self.lives_remaining;
    }
    pub fn stages_complete(&self) -> u32 {
        return self.stages_complete;
    }
    pub fn remove_life(&mut self) {
        self.lives_remaining = self.lives_remaining.saturating_sub(1);
    }
}

