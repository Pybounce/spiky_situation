use bevy::prelude::*;

use crate::{builders::player_builders::PlayerBuilder, common::death::DeathMarker, local_player::LocalPlayer, stage::{stage_builder::{events::{BuildStageEvent, LoadStageEvent}, CurrentStageData}, stage_objects::StageObject}};

use super::spawner::LocalPlayerSpawner;


#[derive(Component)]
pub struct Respawnable {
    pub translation: Vec3,
    pub delay_in_seconds: f64
}

pub fn spawn_player_corpse(
    mut commands: Commands,
    query: Query<&Transform, (With<LocalPlayer>, With<DeathMarker>)>,
    player_builder: Res<PlayerBuilder>
) {
    for transform in &query {
        let mut corpse = commands.spawn(());
        player_builder.build_player_corpse(&mut corpse, transform.translation + Vec3::new(0.0, 0.0, 100.0));
    }
}

pub fn trigger_dead_local_player_respawn(
    mut commands: Commands,
    query: Query<&Respawnable, (With<LocalPlayer>, With<DeathMarker>)>,
    time: Res<Time>,
    stage_data_opt: Option<Res<CurrentStageData>>,
    mut load_event_writer: EventWriter<LoadStageEvent>,
    mut build_event_writer: EventWriter<BuildStageEvent>,
) {
    //if let Some(stage_data) = stage_data_opt {
    //    if let Ok(_) = &query.get_single() {
    //        load_event_writer.send(LoadStageEvent {stage_id: stage_data.stage_id});
    //        build_event_writer.send(BuildStageEvent {stage_id: stage_data.stage_id});
    //    }
    //}

    for respawnable in &query {
        commands.spawn((LocalPlayerSpawner {
            spawn_time: time.elapsed_seconds_f64() + respawnable.delay_in_seconds,
            translation: respawnable.translation,
        }, StageObject));
    }
}