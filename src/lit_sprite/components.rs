
use avian2d::parry::utils::hashmap::HashMap;
use bevy::prelude::*;

#[derive(Resource)]
pub struct LitSpriteDb {
    pub mesh_handle: Handle<Mesh>,
    pub material_cache: HashMap<(AssetId<Image>, AssetId<Image>), Handle<ColorMaterial>>
}