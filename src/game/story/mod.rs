
use bevy::{platform::collections::HashSet, prelude::*};
use serde::{Deserialize, Serialize};


#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct StorySave {
    pub save_id: usize,
    pub completed_levels: HashSet<usize>,
}

impl StorySave {
    pub fn new(save_id: usize) -> Self {
        return Self {
            save_id,
            completed_levels: HashSet::new(),
        };
    }
}