
use bevy::{
    asset::{io::Reader, ron, AssetLoader, LoadContext}, platform::collections::HashMap, prelude::*, reflect::TypePath
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::pair_map::PairMap;

#[derive(Asset, TypePath, Debug, Deserialize, Serialize)]
pub struct Level {
    pub spawn_stage_id: usize,
    /// (stageId, gatewayId)
    pub gateway_pairs: PairMap<(usize, usize)>
}



#[derive(Default)]
pub struct LevelLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LevelLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),

}


impl AssetLoader for LevelLoader {
    type Asset = Level;
    type Settings = ();
    type Error = LevelLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let custom_asset = ron::de::from_bytes::<Level>(&bytes)?;
            Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["level"]
    }

}