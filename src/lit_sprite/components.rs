
use avian2d::parry::utils::hashmap::HashMap;
use bevy::prelude::*;

use crate::lit_sprite::global_components::LitSpriteMaterial;

#[derive(Resource)]
pub struct LitSpriteDb {
    pub mesh_handle: Handle<Mesh>,
    pub material_cache: HashMap<(Option<AssetId<Image>>, Option<AssetId<Image>>), Handle<LitSpriteMaterial>>
}