
use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use rand::Rng;

use crate::{builders::player_builders::PlayerBuilder, common::death::DeathMarker, databases::splat_db::SplatDb, local_player::LocalPlayer, shaders::splat::SplatMaterial};


#[derive(Component)]
pub struct Respawnable {
    pub translation: Vec3,
    pub delay_in_seconds: f64
}

pub fn spawn_player_corpse(
    mut commands: Commands,
    query: Query<&Transform, (With<LocalPlayer>, Added<DeathMarker>)>,
    player_builder: Res<PlayerBuilder>,
) {
    for transform in &query {
        let mut corpse = commands.spawn(());
        player_builder.build_player_corpse(&mut corpse, transform.translation + Vec3::new(0.0, 0.0, 100.0));
    }
}


