
use bevy::{prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{AlphaMode2d, Material2d}};

#[derive(Component)]
pub struct LitSprite {
    pub albedo_texture: Option<Handle<Image>>,
    pub specular_texture: Option<Handle<Image>>,
    pub rect: Option<Rect>,
}


#[derive(Component, Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct LitSpriteMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub albedo_texture: Option<Handle<Image>>,
    #[texture(2)]
    #[sampler(3)]
    pub specular_texture: Option<Handle<Image>>,
    #[uniform(4)]
    pub uv_rect: Vec4
}


impl Material2d for LitSpriteMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shaders/lit_sprite.wgsl".into();
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}