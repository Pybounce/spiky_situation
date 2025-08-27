
use avian2d::prelude::Collider;
use bevy::prelude::*;
use avian2d::prelude::*;
use crate::{common::{animated_sprite::SpriteAnimator, physics::layers::GamePhysicsLayer, splat::SplatProvider}, obstacles::InstantKiller, stage::{stage_builder::{stage_asset, stage_creator::{StageCreator, TILE_SIZE, TILE_SIZE_HALF}}, stage_objects::tiles::TileBundle}};


const PRESSURE_SPIKE_DELAY: f32 = 0.3;

#[derive(Component)]
pub struct PressureSpike {
    triggered: bool,
    timer: Timer
}

impl PressureSpike {
    pub fn new(delay_seconds: f32) -> Self {
        return Self {
            triggered: false,
            timer: Timer::from_seconds(delay_seconds, TimerMode::Once)
        };
    }
}

pub struct PressureSpikeBuilder;

impl PressureSpikeBuilder {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, atlas_rects: Vec<Rect>, pressure_spike: &stage_asset::PressureSpike) {
        let mut mask = LayerMask(GamePhysicsLayer::StageObject.to_bits());
        mask.add(GamePhysicsLayer::Player.to_bits());

        commands.spawn((
            TileBundle::new(stage_creator, pressure_spike.grid_pos, atlas_rects[0], pressure_spike.rotation, stage_creator.object_tilemap),
            SpriteAnimator::new_non_repeating(50, atlas_rects),
            PressureSpike::new(PRESSURE_SPIKE_DELAY),
            RigidBody::Static,
            SplatProvider {
                translation_offset: Vec2::new(0.0, -(TILE_SIZE_HALF * 0.6)),
            },
            children![(
                Transform::from_translation(Vec3::new(0.0, TILE_SIZE_HALF * 0.6, 0.0)),
                Collider::rectangle(TILE_SIZE * 0.8, TILE_SIZE * 0.4),
                CollisionLayers::new(GamePhysicsLayer::StageObject, mask),
                CollisionEventsEnabled,
            )]
        ));
    }

}


pub fn trigger_pressure_spikes(
    trigger_query: Query<&CollidingEntities>,
    mut pressure_spike_query: Query<&mut PressureSpike>
) {
    for colliding_entities in &trigger_query {
        for colliding_entity in colliding_entities.iter() {
            if let Ok(mut pressure_spike) = pressure_spike_query.get_mut(*colliding_entity) {
                if pressure_spike.triggered == false {
                    pressure_spike.triggered = true;
                }
            }
        }
    }
}

pub fn tick_pressure_spikes(
    mut query: Query<(Entity, &mut PressureSpike, &mut SpriteAnimator)>,
    mut commands: Commands,
    time: Res<Time>
) {
    for (e, mut pressure_spike, mut animator) in &mut query {
        if pressure_spike.triggered && !pressure_spike.timer.finished() {
            pressure_spike.timer.tick(time.delta());
            if pressure_spike.timer.remaining() <= animator.duration() {
                animator.play_or_continue();    // TODO: Click sound here
            }
            if pressure_spike.timer.just_finished() {
                commands.entity(e).try_insert(InstantKiller);   // TODO: Blade release sound here
            }
        }
    }
}

// TODO: A super satisfying sound for these guys!
// When you step on them, a click sound
// Then when the animation plays and blades come out, a sword unsheething kind of sound. 