
use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use rand::Rng;

use crate::{common::death::DeathMarker, databases::splat_db::{SplatDb, SplatType}, shaders::splat::SplatMaterial};

#[derive(Component)]
pub struct SplatOnDeath;


#[derive(Component)]
pub struct SplatProvider {
    /// Instead of using the Entity's translation to calc splat direction, use translation + offset
    pub translation_offset: Vec2
}

pub fn apply_splat_on_death(
    mut commands: Commands,
    emitter_query: Query<(&GlobalTransform, &DeathMarker), (With<SplatOnDeath>, Added<DeathMarker>)>,
    killer_query: Query<(&GlobalTransform, Option<&SplatProvider>)>,
    mut materials: ResMut<Assets<SplatMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    splat_db: Res<SplatDb>
) {
    let RAD_45 = std::f32::consts::PI / 4.0;

    for (transform, death_mark) in &emitter_query {

        let splat_rotation = match death_mark.killed_by {
            Some(killed_by) => {
                if let Ok((killer_transform, splat_provider_opt)) = killer_query.get(killed_by) {
                    let killer_pos = match splat_provider_opt {
                        Some(provider) => killer_transform.translation() + (killer_transform.rotation() * provider.translation_offset.extend(0.0)),
                        None => killer_transform.translation(),
                    };

                    let splat_emitter_pos = transform.translation();
                    let direction = (splat_emitter_pos - killer_pos).truncate().normalize_or_zero();
                    let angle = direction.y.atan2(direction.x);
                    //let random_offset = rng.gen_range(-RAD_45..RAD_45);
                    Quat::from_rotation_z(angle - (RAD_45 * 2.0))
                } else { Quat::IDENTITY }
            },
            None => Quat::IDENTITY,
        };

        build_splat(&mut commands, &splat_db, &mut materials, &mut meshes, &time, splat_rotation, transform.translation(), SplatType::Radial);
        build_splat(&mut commands, &splat_db, &mut materials, &mut meshes, &time, splat_rotation, transform.translation(), SplatType::Long);


    }
}

pub fn build_splat(
    commands: &mut Commands, 
    splat_db: &Res<SplatDb>, 
    materials: &mut ResMut<Assets<SplatMaterial>>, 
    meshes: &mut ResMut<Assets<Mesh>>, 
    time: &Res<Time>, 
    splat_rotation: Quat, 
    pos: Vec3,
    splat_type: SplatType
) {
    let mut rng = rand::thread_rng();

    let Some((splat_tex, splat_rect, origin_offset)) = splat_db.random_of_type(splat_type) else { return };

    let uv_rect = Vec4::new(splat_rect.min.x / 1024.0, splat_rect.min.y / 1024.0, splat_rect.max.x / 1024.0, splat_rect.max.y / 1024.0);

    let splat_mat = materials.add(SplatMaterial {
        current_time: time.elapsed_secs(),
        texture: splat_tex,
        uv_rect: uv_rect,
        brightness: rng.gen_range(0.8..1.2)
    });
    let mesh = meshes.add(Mesh::from(Rectangle::default()));
    let translation = pos - (splat_rotation * origin_offset.extend(0.0));
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