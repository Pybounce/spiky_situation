use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

use crate::common::death::DeathMarker;

#[derive(Component)]
pub struct Fragile;

pub fn break_fragiles(
    mut collision_events: EventReader<CollisionEvent>,
    fragile_query: Query<(Entity, Option<&FragileShield>), With<Fragile>>,
    mut commands: Commands
) {
    for collision_event in collision_events.read() {
        let (entity1, entity2) = match collision_event {
            CollisionEvent::Started(e1, e2, _) => { (*e1, *e2) },
            CollisionEvent::Stopped(_, _, _) => { continue; },
        };

        if let Ok((e, fs)) = fragile_query.get(entity1) {
            fragile_hit(&mut commands, e, fs, entity2);
        }
        if let Ok((e, fs)) = fragile_query.get(entity2) {
            fragile_hit(&mut commands, e, fs, entity1);
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