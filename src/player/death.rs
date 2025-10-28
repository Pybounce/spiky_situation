
use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use rand::Rng;

use crate::{builders::player_builders::PlayerBuilder, common::death::DeathMarker, databases::splat_db::SplatDb, local_player::LocalPlayer, shaders::splat::SplatMaterial, stage::stage_builder::{events::BuildStageEvent, CurrentStageData}};


#[derive(Component)]
pub struct Respawnable {
    pub translation: Vec3,
    pub delay_in_seconds: f64
}

pub fn spawn_player_corpse(
    mut commands: Commands,
    query: Query<&Transform, (With<LocalPlayer>, Added<DeathMarker>)>,
    player_builder: Res<PlayerBuilder>,
    current_stage_data_opt: Option<Res<CurrentStageData>>,
    mut build_stage_writer: EventWriter<BuildStageEvent>
) {
    let Some(current_stage_data) = current_stage_data_opt else { return; };

    for transform in &query {
        let mut corpse = commands.spawn(());
        player_builder.build_player_corpse(&mut corpse, transform.translation + Vec3::new(0.0, 0.0, 100.0));
        build_stage_writer.write(BuildStageEvent { stage_id: current_stage_data.stage_id, gateway_id_opt: current_stage_data.gateway_id_opt });
    }
}


