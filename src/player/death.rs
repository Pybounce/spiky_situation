
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

pub fn player_splat(
    mut commands: Commands,
    query: Query<&Transform, (With<LocalPlayer>, Added<DeathMarker>)>,
    mut materials: ResMut<Assets<SplatMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    splat_db: Res<SplatDb>
) {
    let mut rng = rand::thread_rng();

    for transform in &query {
        for _ in 0..1 {
            let (splat_tex, splat_rect) = splat_db.random_radial();

            let uv_rect = Vec4::new(splat_rect.min.x / 1024.0, splat_rect.min.y / 1024.0, splat_rect.max.x / 1024.0, splat_rect.max.y / 1024.0);

            let splat_mat = materials.add(SplatMaterial {
                current_time: time.elapsed_secs(),
                texture: splat_tex,
                uv_rect: uv_rect,
                brightness: rng.gen_range(0.8..1.2)
            });
            let mesh = meshes.add(Mesh::from(Rectangle::default()));

            commands.spawn((
                Mesh2d(mesh),
                MeshMaterial2d(splat_mat),
                Transform { 
                    translation: transform.translation - (Vec3::Z * 10.0), 
                    scale: Vec3::new(splat_rect.width(), splat_rect.height(), 1.0),
                    rotation: Quat::from_rotation_z(rng.gen_range(0..4) as f32 * FRAC_PI_2 * 0.0),
                    ..default()
                },
            ));
        }


    }
}

