
use avian2d::prelude::CollisionStarted;
use bevy::prelude::*;
use bevy_seedling::{SeedlingPlugin, prelude::{InstantSeconds, PlaybackState}, sample::PlaybackSettings};

pub struct AudioPlugin;


impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SeedlingPlugin::default());
    }
}


// Probably makes sense to have a PlayAudioOnTouch(name)
// Bounce on bouncy is fine but not great, what if 2 bouncy things need different sounds