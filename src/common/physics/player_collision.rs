
use avian2d::prelude::{CollisionEnded, CollisionStarted};
use bevy::prelude::*;

use crate::local_player::LocalPlayer;



/// ***IMPROVEMENT HERE TEMP*** ///
/// Can just add logic to ManyCollidingEntities that contains a set for current colliding, newly colliding, and just ended colliding
/// Much the same as the custom keyboard/controller player input I made 
/// 




#[derive(Event)]
pub struct PlayerCollisionStarted {
    pub player: Entity, 
    pub other: Entity
}

#[derive(Event)]
pub struct PlayerCollisionEnded {
    pub player: Entity, 
    pub other: Entity
}



pub fn raise_player_collision_started(
    mut collision_started_reader: EventReader<CollisionStarted>,
    mut collision_started_writer: EventWriter<PlayerCollisionStarted>,
    player_query: Query<(), With<LocalPlayer>>
) {
    for collision_event in collision_started_reader.read() {
        if let Ok(_) = player_query.get(collision_event.0) {
            collision_started_writer.write(PlayerCollisionStarted { player: collision_event.0, other: collision_event.1 });
        }
        if let Ok(_) = player_query.get(collision_event.1) {
            collision_started_writer.write(PlayerCollisionStarted { player: collision_event.1, other: collision_event.0 });
        }
    }
}

pub fn raise_player_collision_ended(
    mut collision_ended_reader: EventReader<CollisionEnded>,
    mut collision_ended_writer: EventWriter<PlayerCollisionEnded>,
    player_query: Query<(), With<LocalPlayer>>
) {
    for collision_event in collision_ended_reader.read() {
        if let Ok(_) = player_query.get(collision_event.0) {
            collision_ended_writer.write(PlayerCollisionEnded { player: collision_event.0, other: collision_event.1 });
        }
        if let Ok(_) = player_query.get(collision_event.1) {
            collision_ended_writer.write(PlayerCollisionEnded { player: collision_event.1, other: collision_event.0 });
        }
    }
}