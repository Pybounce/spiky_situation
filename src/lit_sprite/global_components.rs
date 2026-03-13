
use bevy::prelude::*;

#[derive(Component)]
pub struct LitSprite {
    pub albedo_texture: Handle<Image>,
    pub specular_texture: Handle<Image>,
    pub rect: Option<Rect>,
}