
use bevy::{prelude::*, sprite::Material2dPlugin};

use crate::lit_sprite::{global_components::LitSpriteMaterial, systems::{handle_new_lit_sprites, init_default_lit_sprite}};

mod components;
mod systems;
pub mod global_components;

pub struct LitSpritePlugin;


impl Plugin for LitSpritePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Material2dPlugin::<LitSpriteMaterial>::default())
            .add_systems(Startup, init_default_lit_sprite)
            .add_systems(Update, handle_new_lit_sprites);
        //app.sub_app_mut(RenderApp)
        //   .add_systems(ExtractSchedule, extract_lighting_out_buffer);
    }
}