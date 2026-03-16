
use bevy::prelude::*;
use bevy_seedling::prelude::{EffectsQuery, SampleEffects, Volume, VolumeNode};
use avian2d::prelude::LinearVelocity;

use crate::{ground::Grounded, local_player::MAX_HORIZONTAL_SPEED};

pub fn control_player_footstep_audio(
    query: Query<(&SampleEffects, &LinearVelocity, Option<&Grounded>)>,
    mut volume_query: Query<&mut VolumeNode>,
) {
    for (effects, linvel, grounded_opt) in query.iter() {

        let Ok(mut volume) = volume_query.get_effect_mut(effects) else { continue; };

        let mut target_volume = 0.0;
        let max_volume = 1.0;
        if grounded_opt.is_some() {
            target_volume = 0.0.lerp(max_volume, linvel.x.abs() / MAX_HORIZONTAL_SPEED);
        }
        volume.volume = Volume::Linear(target_volume);
    }
}

