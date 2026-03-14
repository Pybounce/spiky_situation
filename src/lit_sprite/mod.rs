
use bevy::{prelude::*, render::extract_resource::ExtractResourcePlugin, sprite::Material2dPlugin};

use crate::lit_sprite::{global_components::{LitSpriteMaterial, SpecularBuffer}, systems::{handle_new_lit_sprites, init_default_lit_sprite, init_default_specular, init_specular_buffer, resize_specular_buffer, update_materials_with_buffer}};

mod components;
mod systems;
pub mod global_components;

pub struct LitSpritePlugin;


impl Plugin for LitSpritePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Material2dPlugin::<LitSpriteMaterial>::default())
            .add_plugins(ExtractResourcePlugin::<SpecularBuffer>::default())
            .add_systems(Startup, (init_default_lit_sprite, init_specular_buffer, init_default_specular))
            .add_systems(Update, (handle_new_lit_sprites, update_materials_with_buffer, resize_specular_buffer));
        //app.sub_app_mut(RenderApp)
        //   .add_systems(ExtractSchedule, extract_lighting_out_buffer);
    }
}