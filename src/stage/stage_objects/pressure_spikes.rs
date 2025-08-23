
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{common::{animated_sprite::{AnimateOnTouch, SpriteAnimator}, splat::SplatProvider}, obstacles::InstantKiller, stage::{stage_builder::{stage_asset, stage_creator::{StageCreator, TILE_SIZE_HALF}}, stage_objects::{tiles::TileBundle, StageObject}}};


// okay so add the thing with an animator and collider
// give it a timer also

// when collider is collided, start timer AND start animation (animation set to the same time)
// as soon as timer ends, add instantkiller component so collider now kills









// *** NEW PLAN ***
// Build out that generic insert_on_delay and remove_on_delay system
// simple.

// yeah ok it's more complex than that but I NEED to have my stuff more generic
// for the love of god I have so many systems for specific things like PhantomBlocks
// it's shit.



// need a way of saying OnCollideInsertWithDelay(c: Component)
// But I may want multiple. hmm.

// so I need to start animation on collision
// and then x seconds after collision, add instantkiller component

const PRESSURE_SPIKE_DELAY: f32 = 0.5;

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
        commands.spawn((
            TileBundle::new(stage_creator, pressure_spike.grid_pos, atlas_rects[0], pressure_spike.rotation, stage_creator.object_tilemap),
            SpriteAnimator::new_non_repeating(50, atlas_rects),
            PressureSpike::new(PRESSURE_SPIKE_DELAY),
            Collider::compound(vec![((Vect::new(0.0, -(TILE_SIZE_HALF * 0.6))), 0.0, Collider::cuboid(TILE_SIZE_HALF * 0.8, TILE_SIZE_HALF * 0.4))]),
            RigidBody::Fixed,
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(Group::GROUP_2, Group::ALL),
            SplatProvider {
                translation_offset: Vec2::new(0.0, -(TILE_SIZE_HALF * 0.6)),
            }
        ));
    }

}


pub fn trigger_pressure_spikes(
    trigger_query: Query<&CollidingEntities>,
    mut pressure_spike_query: Query<&mut PressureSpike>
) {
    for colliding_entities in &trigger_query {
        for colliding_entity in colliding_entities.iter() {
            if let Ok(mut pressure_spike) = pressure_spike_query.get_mut(colliding_entity) {
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
            if pressure_spike.timer.just_finished() {
                animator.play();
                commands.entity(e).try_insert(InstantKiller);
            }
        }
    }
}

// TODO: A super satisfying sound for these guys!
// When you step on them, a click sound
// Then when the animation plays and blades come out, a sword unsheething kind of sound. 