
use bevy::{prelude::*, scene::ron};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::game::endless::components::EndlessRun;

#[derive(Resource, Default)]
pub struct SaveDb {

}


impl SaveDb {
    pub fn save_endless(&self, endless_run: &EndlessRun) {
        let game_save = GameSave::Endless(endless_run.clone());
        if let Some(proj_dirs) = ProjectDirs::from("com", "Skybounce", "Platformer") {
            let path = proj_dirs.config_dir().join("save_files");
            let mut bytes: Vec<u8> = vec![];
            ron::ser::to_writer(&mut bytes, &game_save).unwrap();

            let _ = std::fs::create_dir_all(&path);
            let mut file = std::fs::File::create(&path.join("game_save.dat")).expect("failed to create file for endless save");       
            
            use std::io::Write;
            file.write_all(&bytes).expect("failed to save endless");
        }
    }
    
    pub fn delete_game_save(&self) {
        if let Some(proj_dirs) = ProjectDirs::from("com", "Skybounce", "Platformer") {
            let path = proj_dirs.config_dir().join("save_files").join("game_save.dat");
            let _ = std::fs::remove_file(&path);  
        }
    }

    pub fn get_existing_save(&self) -> Option<GameSave> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "Skybounce", "Platformer") {
            let path = proj_dirs.config_dir().join("save_files").join("game_save.dat");
            if let Ok(bytes) = std::fs::read(path) {
                if let Ok(save) = ron::de::from_bytes::<GameSave>(&bytes) {
                    return Some(save);
                }
            }
        }

        return None;
    }

}



#[derive(Serialize, Deserialize)]
pub enum GameSave {
    Endless(EndlessRun)
}


//pub struct SaveSlot {
//    pub slot_id: u32,
//    pub save_data: GameSave
//}

#[derive(Event)]
pub struct SaveGame;

