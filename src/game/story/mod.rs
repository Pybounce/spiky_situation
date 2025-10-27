
use bevy::{platform::collections::HashSet, prelude::*};
use serde::{Deserialize, Serialize};


#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct StorySave {
    pub save_id: usize,
    pub completed_levels: HashSet<usize>,
}
