
use bevy::{ecs::relationship, prelude::*};
use bevy_rapier2d::prelude::CollisionEvent;

#[derive(Component)]
pub struct ColliderOf(pub Entity);

#[derive(Event)]
pub struct CollisionRemapEvent(CollisionEvent);

pub fn raise_collision_remap_events(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut collision_remap_event_writer: EventWriter<CollisionRemapEvent>,
    collider_of_query: Query<&ColliderOf>,
) {
    for event in collision_event_reader.read() {
        let (e1, e2, flags) = match event {
            CollisionEvent::Started(e1, e2, flags) => (*e1, *e2, *flags),
            CollisionEvent::Stopped(e1, e2, flags) => (*e1, *e2, *flags),
        };

        let new_e1 = collider_of_query.get(e1).map(|c| c.0).unwrap_or(e1);
        let new_e2 = collider_of_query.get(e2).map(|c| c.0).unwrap_or(e2);

        if new_e1 != e1 || new_e2 != e2 {
            let new_event = match event {
                CollisionEvent::Started(_, _, _) => CollisionEvent::Started(new_e1, new_e2, flags),
                CollisionEvent::Stopped(_, _, _) => CollisionEvent::Stopped(new_e1, new_e2, flags),
            };
            collision_remap_event_writer.write(CollisionRemapEvent(new_event));
        }
    }
}


pub fn handle_collision_remap_events(
    mut collision_remap_event_reader: EventReader<CollisionRemapEvent>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    for event in collision_remap_event_reader.read() {
        collision_event_writer.write(event.0);
    }
}
