
use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use rand::Rng;

use crate::{common::death::DeathMarker, databases::splat_db::SplatDb, shaders::splat::SplatMaterial};

#[derive(Component)]
pub struct SplatOnDeath;



pub fn apply_splat_on_death(
    mut commands: Commands,
    query: Query<&Transform, (With<SplatOnDeath>, Added<DeathMarker>)>,
    mut materials: ResMut<Assets<SplatMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    splat_db: Res<SplatDb>
) {
    let mut rng = rand::thread_rng();
    for transform in &query {
        for _ in 0..2 {
            let (splat_tex, splat_rect, origin_offset) = splat_db.random_radial();

            let uv_rect = Vec4::new(splat_rect.min.x / 1024.0, splat_rect.min.y / 1024.0, splat_rect.max.x / 1024.0, splat_rect.max.y / 1024.0);

            let splat_mat = materials.add(SplatMaterial {
                current_time: time.elapsed_secs(),
                texture: splat_tex,
                uv_rect: uv_rect,
                brightness: rng.gen_range(0.8..1.2)
            });
            let mesh = meshes.add(Mesh::from(Rectangle::default()));
            let translation = transform.translation - origin_offset.extend(0.0);
            commands.spawn((
                Mesh2d(mesh),
                MeshMaterial2d(splat_mat),
                Transform { 
                    translation: translation.with_z(-10.0), 
                    scale: Vec3::new(splat_rect.width(), splat_rect.height(), 1.0),
                    rotation: Quat::from_rotation_z(rng.gen_range(0..4) as f32 * FRAC_PI_2 * 0.0),
                    ..default()
                },
            ));
        }


    }
}
