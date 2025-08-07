
use bevy::{prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{AlphaMode2d, Material2d}};

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct SplatMaterial {
    #[uniform(0)]
    pub current_time: f32,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material2d for SplatMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/splat.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }


}