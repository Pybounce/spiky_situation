use bevy::prelude::*;
use avian2d::prelude::*;

use crate::common::death::DeathMarker;

#[derive(Component)]
pub struct Fragile;

pub fn break_fragiles(
    mut collision_events: EventReader<CollisionStarted>,
    fragile_query: Query<(Entity, Option<&FragileShield>), With<Fragile>>,
    mut commands: Commands
) {
    for collision_event in collision_events.read() {
        if let Ok((e, fs)) = fragile_query.get(collision_event.0) {
            fragile_hit(&mut commands, e, fs, collision_event.1);
        }
        if let Ok((e, fs)) = fragile_query.get(collision_event.1) {
            fragile_hit(&mut commands, e, fs, collision_event.0);
        }
    }
}

// Ok so this is pretty dumb but I can't be fucked to deal with the physics system right now
// If you spawn a fragile thing (such as a sawblade), inside of something that can kill it (such as ground/saw dispenser), it should not break.
// So this just makes it so the first collision entry doesn't kill it.
#[derive(Component)]
pub struct FragileShield;

fn fragile_hit(commands: &mut Commands, entity: Entity, shield: Option<&FragileShield>, hit_by: Entity) {
    match shield {
        Some(_) => commands.entity(entity).remove::<FragileShield>(),
        None => commands.entity(entity).try_insert(DeathMarker::killed_by(hit_by)),
    };
}