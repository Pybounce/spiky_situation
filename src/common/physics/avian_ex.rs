
use bevy::{platform::collections::HashMap, prelude::*};
use avian2d::prelude::*;

#[derive(Component, Default)]
pub struct ManyCollidingEntities {
    colliding_entities: HashMap<Entity, u32>
}

impl ManyCollidingEntities {
    fn add_entity(&mut self, entity: &Entity) {
        if let Some(count) = self.colliding_entities.get_mut(entity) {
            *count += 1;
        }
        else {
            self.colliding_entities.insert(*entity, 1);
        }
    }
    fn remove_entity(&mut self, entity: &Entity) {
        if let Some(count) = self.colliding_entities.get_mut(entity) {
            *count = count.saturating_sub(1);
            if *count == 0 {
                self.colliding_entities.remove(entity);
            }
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        return self.colliding_entities.keys();
    }
}

pub fn handle_many_colliding_entities(
    mut collision_start_events: EventReader<CollisionStarted>,
    mut collision_end_events: EventReader<CollisionEnded>,
    mut query: Query<(Entity, &mut ManyCollidingEntities)>,
) {
    for (entity, mut colliding_entities) in &mut query {
        for collision_started in collision_start_events.read() {
            if collision_started.0 == entity {
                colliding_entities.add_entity(&collision_started.1);
            }
            else if collision_started.1 == entity {
                colliding_entities.add_entity(&collision_started.0);
            }
        }
        for collision_ended in collision_end_events.read() {
            if collision_ended.0 == entity {
                colliding_entities.remove_entity(&collision_ended.1);
            }
            else if collision_ended.1 == entity {
                colliding_entities.remove_entity(&collision_ended.0);
            }
        }
    }
}

enum CollisionEvent {
    Started(CollisionStarted),
    Ended(CollisionEnded)
}

#[derive(Event)]
pub struct CollisionRemapEvent(CollisionEvent);

pub fn raise_collision_remap_events(
    mut collision_start_reader: EventReader<CollisionStarted>,
    mut collision_end_reader: EventReader<CollisionEnded>,
    mut collision_remap_event_writer: EventWriter<CollisionRemapEvent>,
    collider_of_query: Query<&ColliderOf>,
) {
    for started_event in collision_start_reader.read() {
        let new_e1 = collider_of_query.get(started_event.0).map(|c| c.body).unwrap_or(started_event.0);
        let new_e2 = collider_of_query.get(started_event.1).map(|c| c.body).unwrap_or(started_event.1);

        if new_e1 != started_event.0 || new_e2 != started_event.1 {
            let new_event = CollisionEvent::Started(CollisionStarted(new_e1, new_e2));
            collision_remap_event_writer.write(CollisionRemapEvent(new_event));
        }
    }
    for ended_event in collision_end_reader.read() {
        let new_e1 = collider_of_query.get(ended_event.0).map(|c| c.body).unwrap_or(ended_event.0);
        let new_e2 = collider_of_query.get(ended_event.1).map(|c| c.body).unwrap_or(ended_event.1);

        if new_e1 != ended_event.0 || new_e2 != ended_event.1 {
            let new_event = CollisionEvent::Ended(CollisionEnded(new_e1, new_e2));
            collision_remap_event_writer.write(CollisionRemapEvent(new_event));
        }
    }
}


pub fn handle_collision_remap_events(
    mut collision_remap_event_reader: EventReader<CollisionRemapEvent>,
    mut collision_start_writer: EventWriter<CollisionStarted>,
    mut collision_end_writer: EventWriter<CollisionEnded>,
) {
    for event in collision_remap_event_reader.read() {
        match event.0 {
            CollisionEvent::Started(collision_started) =>  { collision_start_writer.write(collision_started); },
            CollisionEvent::Ended(collision_ended) =>  { collision_end_writer.write(collision_ended); },
        };
    }
}