
use std::{env, path::PathBuf};

use bevy::{prelude::*, scene::ron};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::{common::pair_map::PairMap, databases::save_db::GameSave, game::story::StorySave, stage::stage_builder::stage_asset::Stage};

#[derive(Clone, Serialize, Deserialize)]
pub struct Level {
    pub name: String,
    /// (stageId, gatewayId)
    pub gateway_pairs: PairMap<(usize, usize)>,
    pub spawn_stage_id: usize,
}

#[derive(Resource, Default)]
pub struct GameDb {

}

impl GameDb {
    pub fn save_story(&self, story_save: &StorySave) -> bool {
        let game_save = GameSave::Story(story_save.clone());
        if let Ok(path) = PathHelper::gamesaves_path() {
            let mut bytes: Vec<u8> = vec![];
            ron::ser::to_writer(&mut bytes, &game_save).unwrap();

            if GameDb::write_to_file(&path, &PathHelper::gamesave_filename(story_save.save_id), &bytes) {
                return true;
            }
        }

        error!("failed to save story");
        return false;
    }

    pub fn load_gamesave(&self, gamesave_id: usize) -> Option<GameSave> {
        if let Ok(path) = PathHelper::gamesave_path(gamesave_id) {
            if let Ok(bytes) = std::fs::read(path) {
                if let Ok(save) = ron::de::from_bytes::<GameSave>(&bytes) {
                    return Some(save);
                }
            }
        }
        return None;
    }

    pub fn load_all_levels(&self) -> Vec<Level> {
        let mut levels = Vec::<Level>::new();

        for level_id in self.all_level_ids() {
            if let Some(level_data) = self.load_level(level_id) {
                levels.push(level_data);
            }
        }
        return levels;
    }

    pub fn load_level(&self, level_id: usize) -> Option<Level> {
        if let Ok(path) = PathHelper::level_path(level_id) {
            if let Ok(bytes) = std::fs::read(path) {
                if let Ok(data) = ron::de::from_bytes::<Level>(&bytes) {
                    return Some(data);
                }
            }
        }
        return None;
    }

    pub fn load_stage(&self, level_id: usize) -> Option<Stage> {
        if let Ok(path) = PathHelper::level_path(level_id) {
            if let Ok(bytes) = std::fs::read(path) {
                if let Ok(data) = ron::de::from_bytes::<Stage>(&bytes) {
                    return Some(data);
                }
            }
        }
        return None;
    }

    pub fn all_level_ids(&self) -> Vec<usize> {
        let mut ids = Vec::<usize>::new();

        if let Ok(path) = PathHelper::levels_path() {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if path.extension().map_or(true, |ext| ext != "dat") { continue; }

                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            let id_string = name.strip_prefix("level_").unwrap().strip_suffix(".dat").unwrap();
                            if let Ok(id) = id_string.parse::<usize>() {
                                ids.push(id);
                            }
                        }
                    }
                }
            }
        }
        return ids;
    }
}

impl GameDb {
    fn write_to_file(path: &PathBuf, file_name: &str, bytes: &Vec<u8>) -> bool {
        let _ = std::fs::create_dir_all(&path);
        let Ok(mut file) = std::fs::File::create(&path.join(file_name)) else { return false; };
        use std::io::Write;
        return file.write_all(&bytes).is_ok();
    }
}

pub struct PathHelper;

impl PathHelper {
    fn local_path() -> Result<PathBuf, ()> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "skybounce", "spiky_situation") {
            return Ok(proj_dirs.config_dir().to_path_buf());
        }
        return Err(());
    }
    fn assets_path() -> Result<PathBuf, ()> {
        let dir = if let Ok(manifest_dir) = env::var("BEVY_ASSET_ROOT") {
            PathBuf::from(manifest_dir)
        } else if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            PathBuf::from(manifest_dir)
        } else {
            env::current_exe()
                .map(|path| path.parent().map(ToOwned::to_owned).unwrap())
                .unwrap()
        }.join("assets");

        return Ok(dir);
        
    }
    fn gamesaves_path() -> Result<PathBuf, ()> {
        if let Ok(local_path) = PathHelper::local_path() {
            return Ok(local_path.join("gamesaves"));
        }
        return Err(());
    }
    fn gamesave_path(gamesave_id: usize) -> Result<PathBuf, ()> {
        if let Ok(gamesaves_path) = PathHelper::gamesaves_path() {
            return Ok(gamesaves_path.join(PathHelper::gamesave_filename(gamesave_id)));
        }
        return Err(());
    }
    fn custom_levels_path() -> Result<PathBuf, ()> {
        if let Ok(local_path) = PathHelper::local_path() {
            return Ok(local_path.join("custom").join("levels"));
        }
        return Err(());
    }
    fn custom_stages_path() -> Result<PathBuf, ()> {
        if let Ok(local_path) = PathHelper::local_path() {
            return Ok(local_path.join("custom").join("stages"));
        }
        return Err(());
    }
    fn stages_path() -> Result<PathBuf, ()> {
        if let Ok(assets_path) = PathHelper::assets_path() {
            return Ok(assets_path.join("stages"));
        }
        return Err(());
    }
    fn stage_path(stage_id: usize) -> Result<PathBuf, ()> {
        if let Ok(stages_path) = PathHelper::stages_path() {
            return Ok(stages_path.join(PathHelper::stage_filename(stage_id)));
        }
        return Err(());
    }
    fn levels_path() -> Result<PathBuf, ()> {
        if let Ok(assets_path) = PathHelper::assets_path() {
            return Ok(assets_path.join("levels"));
        }
        return Err(());
    }
    fn level_path(level_id: usize) -> Result<PathBuf, ()> {
        if let Ok(levels_path) = PathHelper::levels_path() {
            return Ok(levels_path.join(PathHelper::level_filename(level_id)));
        }
        return Err(());
    }
    fn gamesave_filename(gamesave_id: usize) -> String {
        return format!("gamesave_{}.dat", gamesave_id);
    }
    fn level_filename(level_id: usize) -> String {
        return format!("level_{}.dat", level_id);
    }
    fn stage_filename(stage_id: usize) -> String {
        return format!("stage_{}.dat", stage_id);
    }
}

// read level data
// write level data

// read stage data
// write stage data

// read gamesave data
// write gamesave data