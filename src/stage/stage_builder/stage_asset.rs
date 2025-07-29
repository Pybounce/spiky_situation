

use bevy::{
    asset::{io::Reader, ron, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
};
use serde::{ Deserialize, Serialize};
use thiserror::Error;

use super::stage_creator::TILE_SIZE;

#[derive(Asset, TypePath, Debug, Deserialize, Serialize)]
pub struct Stage {
    pub id: usize,
    pub terrain_theme: TerrainTheme,
    pub ground_tiles: Vec<GroundTile>,
    pub spikes: Vec<Spike>,
    pub half_saws: Vec<HalfSaw>,
    pub springs: Vec<Spring>,
    pub lock_blocks: Vec<LockBlock>,
    pub keys: Vec<Key>,
    pub interval_blocks: Vec<IntervalBlock>,
    pub saw_shooter_blocks: Vec<SawShooterBlock>,
    pub phantom_blocks: Vec<PhantomBlock>,
    pub checkpoints: Vec<Checkpoint>,
    pub grid_width: usize,
    pub grid_height: usize,
    pub spawn_grid_pos: Vec2,
    pub goal_grid_pos: Vec2
}

impl Stage {
    pub fn new(id: usize, grid_size: IVec2) -> Self {
        Self {
            id: id,
            ground_tiles: vec![],
            spikes: vec![],
            half_saws: vec![],
            springs: vec![],
            lock_blocks: vec![],
            keys: vec![],
            interval_blocks: vec![],
            saw_shooter_blocks: vec![],
            phantom_blocks: vec![],
            checkpoints: vec![],
            grid_width: grid_size.x as usize,
            grid_height: grid_size.y as usize,
            spawn_grid_pos: Vec2::default(),
            goal_grid_pos: Vec2::new(100.0 * TILE_SIZE, 0.0),
            terrain_theme: TerrainTheme::Grass,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TerrainTheme {
    Grass,
    Snow,
    Sand,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroundTile {
    pub grid_pos: Vec2,
    pub tilemap_index: usize
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Spike {
    pub grid_pos: Vec2,
    pub rotation: f32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HalfSaw {
    pub grid_pos: Vec2,
    pub rotation: f32,
    pub movement_path_opt: Option<MovementPath>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Checkpoint {
    pub grid_pos: Vec2
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Spring {
    pub grid_pos: Vec2,
    pub rotation: f32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LockBlock {
    pub grid_pos: Vec2,
    pub trigger_id: usize
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Key {
    pub grid_pos: Vec2,
    pub trigger_id: usize
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IntervalBlock {
    pub grid_pos: Vec2,
    pub is_active: bool
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PhantomBlock {
    pub grid_pos: Vec2
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SawShooterBlock {
    pub grid_pos: Vec2,
    pub rotation: f32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MovementPath {
    pub grid_offsets: Vec<Vec2>,
    pub speed: f32
}

#[derive(Default)]
pub struct StageLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum StageLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),

}


impl AssetLoader for StageLoader {
    type Asset = Stage;
    type Settings = ();
    type Error = StageLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let custom_asset = ron::de::from_bytes::<Stage>(&bytes)?;
            Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["stage"]
    }

}