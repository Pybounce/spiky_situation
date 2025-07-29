use bevy::prelude::*;
use bevy_rapier2d::prelude::CollidingEntities;

#[derive(Component)]
pub struct DeathMarker;

#[derive(Component)]
pub struct DelayedDeathMarker {
    delay: Timer
}

impl DelayedDeathMarker {
    pub fn from_secs(delay: f32) -> Self {
        return Self {
            delay: Timer::from_seconds(delay, TimerMode::Once)
        }
    }
}

#[derive(Component)]
pub struct Killable;

pub fn despawn_death_marked(
    mut commands: Commands,
    query: Query<Entity, With<DeathMarker>>
) {
    for e in &query {
        commands.entity(e).despawn();
    }
}

pub fn delay_death_marked(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DelayedDeathMarker), Without<DeathMarker>>,
    time: Res<Time>
) {
    for (e, mut delayed_death_marker) in &mut query {
        delayed_death_marker.delay.tick(time.delta());
        if delayed_death_marker.delay.finished() {
            commands.entity(e).try_insert(DeathMarker);
        }
    }
}


#[derive(Component)]
pub struct DeathMarkOnTouch;

pub fn check_touched_by_death(
    query: Query<&CollidingEntities>,
    death_marked_on_touch_query: Query<Entity, With<DeathMarkOnTouch>>,
    mut commands: Commands
) {
    for colliding_entities in &query {
        for colliding_entity in colliding_entities.iter() {
            if let Ok(entity) = death_marked_on_touch_query.get(colliding_entity) {
                commands.entity(entity).try_insert(DeathMarker);
            }
        }
    }
}