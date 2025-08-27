
use bevy::prelude::*;
use avian2d::prelude::*;



#[derive(Component)]
pub struct Trigger {
    pub trigger_id: usize
}

#[derive(Component)]
pub struct TriggerOnTouch;

#[derive(Component)]
pub struct Triggerable {
    pub trigger_id: usize
}

#[derive(Event)]
pub struct TriggerEvent {
    pub trigger_id: usize
}

pub fn trigger_on_touch(
    colliding_query: Query<&CollidingEntities>,
    trigger_query: Query<&Trigger, With<TriggerOnTouch>>,
    mut trigger_event_writer: EventWriter<TriggerEvent>
) {
    for colliding_entities in &colliding_query {

        for colliding_entity in colliding_entities.iter() {
            if let Ok(trigger) = trigger_query.get(*colliding_entity) {
                trigger_event_writer.write(TriggerEvent {
                    trigger_id: trigger.trigger_id
                });
            }
        }
    }
}