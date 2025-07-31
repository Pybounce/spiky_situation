
use bevy::prelude::*;

use crate::game::endless::components::EndlessRun;

#[derive(Resource)]
pub struct SaveDb {

}


impl SaveDb {
    pub fn save_endless(&mut self, endless_run: EndlessRun) {
        todo!();
    }

}



pub fn init_save_db(
    mut commands: Commands
) {

}


pub enum GameSave {
    Endless(EndlessGameSave)
}

pub struct EndlessGameSave {

}

pub struct SaveSlot {
    pub slot_id: u32,
    pub save_data: GameSave
}

#[derive(Event)]
pub struct SaveGame;

