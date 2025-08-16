
use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use rand::Rng;

use crate::{common::death::DeathMarker, databases::splat_db::SplatDb, shaders::splat::SplatMaterial};

#[derive(Component)]
pub struct SplatOnDeath;


#[derive(Component)]
pub struct SplatProvider {
    /// Instead of using the Entity's translation to calc splat direction, use translation + offset
    pub translation_offset: Vec2
}


fn angle_from_positive_y(dir: Vec2) -> f32 {
    let dot = dir.dot(-Vec2::Y);
    let mut angle = -(dot - 1.0);
    if dir.x < 0.0 { angle = -angle; }
    return angle * std::f32::consts::FRAC_PI_2; 
}

pub fn apply_splat_on_death(
    mut commands: Commands,
    query: Query<(&GlobalTransform, &DeathMarker, Option<&SplatProvider>), (With<SplatOnDeath>, Added<DeathMarker>)>,
    transform_query: Query<&GlobalTransform>,
    mut materials: ResMut<Assets<SplatMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    splat_db: Res<SplatDb>
) {
    let mut rng = rand::thread_rng();
    for (transform, death_mark, splat_provider_opt) in &query {

        let splat_rotation = match death_mark.killed_by {
            Some(killed_by) => {
                if let Ok(killer_transform) = transform_query.get(killed_by) {
                    let killer_pos = match splat_provider_opt {
                        Some(provider) => killer_transform.translation() + provider.translation_offset.extend(0.0),
                        None => killer_transform.translation(),
                    };
                    let splat_emitter_pos = transform.translation();
                    let direction = (killer_pos - splat_emitter_pos).truncate().normalize_or_zero();
                    let angle = angle_from_positive_y(direction);
                    let snapped_angle = (angle / (std::f32::consts::PI / 4.0)).round() * (std::f32::consts::PI / 4.0);
                    Quat::from_rotation_z(snapped_angle)
                } else { Quat::IDENTITY }
            },
            None => Quat::IDENTITY,
        };

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
            let translation = transform.translation() - (splat_rotation * origin_offset.extend(0.0));
            commands.spawn((
                Mesh2d(mesh),
                MeshMaterial2d(splat_mat),
                Transform { 
                    translation: translation.with_z(-10.0), 
                    scale: Vec3::new(splat_rect.width(), splat_rect.height(), 1.0),
                    rotation: splat_rotation,
                    ..default()
                },
            ));
        }


    }
}
