
use bevy::{platform::collections::HashMap, prelude::*};

use crate::lit_sprite::global_components::{LitSprite, LitSpriteMaterial};

#[derive(Resource)]
pub struct LitSpriteDb {
    pub material_cache: HashMap<LitSpriteMaterialKey, Handle<LitSpriteMaterial>>,    // TODO - Maybe use a weak clone of the handle here
    pub mesh_cache: HashMap<LitSpriteMeshKey, Handle<Mesh>>
}

#[derive(PartialEq, Eq, Hash)]
pub struct LitSpriteMaterialKey {
    pub albedo_id: Option<AssetId<Image>>,
    pub specular_id: Option<AssetId<Image>>,
    pub rect_bits: [u32; 4],
}

impl LitSpriteMaterialKey {
    pub fn from_sprite(sprite: &LitSprite) -> Self {
        let rect = sprite.rect.unwrap_or(Rect::new(0.0, 0.0, 1.0, 1.0));
        return LitSpriteMaterialKey {
            albedo_id: sprite.albedo_texture.as_ref().map(|h| h.id()),
            specular_id: sprite.specular_texture.as_ref().map(|h| h.id()),
            rect_bits: [
                rect.min.x.to_bits(),
                rect.min.y.to_bits(),
                rect.width().to_bits(),
                rect.height().to_bits(),
            ]
        };
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct LitSpriteMeshKey {
    pub size_bits: [u32; 2],
}

impl LitSpriteMeshKey {
    pub fn from_sprite(sprite: &LitSprite) -> Self {
        return Self {
            size_bits: [sprite.size.x.to_bits(), sprite.size.y.to_bits()],
        };
    }
}