
use bevy::prelude::*;
use bevy_seedling::prelude::*;

pub enum GlobalSfx {
    FireCrackling,
    Buzzsaw
}

#[derive(Component)]
pub struct GlobalAudioEmitter(pub GlobalSfx);

