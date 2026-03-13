
use avian2d::parry::utils::hashmap::HashMap;
use bevy::prelude::*;

use crate::lit_sprite::global_components::LitSpriteMaterial;

#[derive(Resource)]
pub struct LitSpriteDb {
    pub mesh_handle: Handle<Mesh>,
    pub material_cache: HashMap<LitSpriteMaterialKey, Handle<LitSpriteMaterial>>    // TODO - Maybe use a weak clone of the handle here
}

#[derive(PartialEq, Eq, Hash)]
pub struct LitSpriteMaterialKey {
    pub albedo_id: Option<AssetId<Image>>,
    pub specular_id: Option<AssetId<Image>>,
    pub rect_bits: [u32; 4],
}