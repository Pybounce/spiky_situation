
use bevy::{diagnostic::FrameCount, prelude::*};
use bevy_seedling::prelude::{EffectsQuery, SampleEffects, Volume, VolumeNode};
use avian2d::prelude::LinearVelocity;

use crate::{audio::{PlaySfxEvent, Sfx}, ground::Grounded, local_player::MAX_HORIZONTAL_SPEED, wall::TouchingWall};


#[derive(Component)]
pub struct WallSlideAudioEmitter(pub Entity);

#[derive(Component)]
pub struct FootstepAudioEmitter(pub Entity);

pub fn control_player_footstep_audio(
    query: Query<(&FootstepAudioEmitter, &LinearVelocity, Option<&Grounded>)>,
    effects_query: Query<&SampleEffects>,
    mut volume_query: Query<&mut VolumeNode>,
) {
    for (emitter, linvel, grounded_opt) in query.iter() {
        let Ok(effects) = effects_query.get(emitter.0) else { continue; };
        let Ok(mut volume) = volume_query.get_effect_mut(effects) else { continue; };

        let mut target_volume = 0.0;
        let max_volume = 1.0;
        if grounded_opt.is_some() {
            target_volume = 0.0.lerp(max_volume, linvel.x.abs() / MAX_HORIZONTAL_SPEED).min(max_volume);
        }
        volume.volume = Volume::Linear(target_volume);
    }
}

pub fn control_player_wall_slide_audio(
    query: Query<(&WallSlideAudioEmitter, &LinearVelocity, Option<&TouchingWall>)>,
    effects_query: Query<&SampleEffects>,
    mut volume_query: Query<&mut VolumeNode>,
) {
    for (emitter, linvel, grounded_opt) in query.iter() {
        let Ok(effects) = effects_query.get(emitter.0) else { continue; };
        let Ok(mut volume) = volume_query.get_effect_mut(effects) else { continue; };

        let mut target_volume = 0.0;
        let max_volume = 0.4;
        if grounded_opt.is_some() {
            target_volume = 0.0.lerp(max_volume, linvel.y.abs() / (MAX_HORIZONTAL_SPEED + 50.0)).min(max_volume);    //TODO: Max horizontal speed should not be here but unlike running, wall slide has no max, it's friction based.
        }
        volume.volume = Volume::Linear(target_volume);
    }
}


// Yes technically the below system apply to all things not just player

pub fn control_player_impact_audio(
    query: Query<&Transform, Or<((Added<TouchingWall>, Without<Grounded>), Added<Grounded>)>>,
    mut event_writer: EventWriter<PlaySfxEvent>,
) {
    for t in query.iter() {
        event_writer.write(PlaySfxEvent {
            sfx: Sfx::PlayerSurfaceHit,
            translation: t.translation,
        });
    }
}