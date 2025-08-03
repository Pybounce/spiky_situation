
use bevy::{prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::Material2d};

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct BackgroundMaterial {

}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
}