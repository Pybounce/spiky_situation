use bevy::prelude::*;
use avian2d::prelude::*;

use crate::common::{death::{DeathMarker, Killable}, physics::avian_ex::ManyCollidingEntities};

#[derive(Component)]
pub struct InstantKiller;


pub fn check_insta_kill_collisions(
    killable_query: Query<(Entity, &ManyCollidingEntities), With<Killable>>,
    killer_query: Query<(), With<InstantKiller>>,
    mut commands: Commands
) {
    for (e, colliding_entities) in &killable_query {

        for colliding_entity in colliding_entities.iter() {
            if let Ok(_) = killer_query.get(*colliding_entity) {
                commands.entity(e).try_insert(DeathMarker::killed_by(*colliding_entity));
            }
        }
    }
}
