use std::hash::Hash;

use bevy::platform::collections::HashMap;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PairMap<T: Clone + Copy + Eq + Hash> {
    map: HashMap<T, T>
}

impl<T: Eq + Clone + Hash + Copy> PairMap<T> {

    pub fn insert_pair(&mut self, left: &T, right: &T) {
        self.map.insert(*left, *right);
        self.map.insert(*right, *left);
    }

    pub fn remove_pairs_containing(&mut self, item: &T) {
        if let Some(other_item) = self.map.remove(item) {
            self.map.remove(&other_item);
        }
    }

    /// Returns the other side of the pair
    pub fn get(&self, item: &T) -> Option<&T> {
        return self.map.get(item);
    }

}


