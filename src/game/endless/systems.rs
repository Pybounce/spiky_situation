
use bevy::prelude::*;

use crate::{databases::save_db::{SaveDb, SaveGame}, game::endless::components::EndlessRun};


pub fn save_endless_game(
    mut save_game_event_reader: EventReader<SaveGame>,
    endless_game_opt: Option<Res<EndlessRun>>,
    mut save_db: ResMut<SaveDb>
) {
    if let Some(endless_run) = endless_game_opt { 
        let mut saved = false;
        for _ in save_game_event_reader.read() {
            if saved { continue; }



        }
    }

}