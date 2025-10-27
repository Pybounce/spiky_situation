
use bevy::{prelude::*, scene::ron};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::game::story::StorySave;

#[derive(Resource, Default)]
pub struct SaveDb {

}


impl SaveDb {
    pub fn save_story(&self, story_save: &StorySave) {
        let game_save = GameSave::Story(story_save.clone());
        if let Some(proj_dirs) = ProjectDirs::from("com", "Skybounce", "Platformer") {
            let path = proj_dirs.config_dir().join("save_files");
            let mut bytes: Vec<u8> = vec![];
            ron::ser::to_writer(&mut bytes, &game_save).unwrap();

            let _ = std::fs::create_dir_all(&path);
            let mut file = std::fs::File::create(&path.join(format!("story_save_{}.dat", story_save.save_id))).expect("failed to create file for story save");       
            
            use std::io::Write;
            file.write_all(&bytes).expect("failed to save story");
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
    Story(StorySave)
}


//pub struct SaveSlot {
//    pub slot_id: u32,
//    pub save_data: GameSave
//}

#[derive(Event)]
pub struct SaveGame;

