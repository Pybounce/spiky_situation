
use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use rand::Rng;

use crate::{common::death::DeathMarker, databases::splat_db::{SplatDb, SplatType}, shaders::splat::SplatMaterial, stage::{stage_builder::CurrentStageData, stage_objects::StageObject}};

#[derive(Event)]
pub struct ClearSplatsEvent;

pub fn clear_splat_events(
    mut clear_splats_reader: EventReader<ClearSplatsEvent>,
    query: Query<Entity, With<Splat>>,
    mut commands: Commands
) {
    for _ in clear_splats_reader.read() {
        for entity in &query {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
pub struct Splat;

#[derive(Component)]
pub struct SplatOnDeath;

const RAD_45: f32  = std::f32::consts::PI / 4.0;
const RAD_90: f32 = std::f32::consts::PI / 2.0;

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
    splat_db: Res<SplatDb>,
) {

    for (transform, death_mark) in &emitter_query {

        let (splat_rot_90, splat_rot_45) = match death_mark.killed_by {
            Some(killed_by) => {
                if let Ok((killer_transform, splat_provider_opt)) = killer_query.get(killed_by) {
                    let killer_pos = match splat_provider_opt {
                        Some(provider) => killer_transform.translation() + (killer_transform.rotation() * provider.translation_offset.extend(0.0)),
                        None => killer_transform.translation(),
                    };

                    let splat_emitter_pos = transform.translation();
                    let direction = (splat_emitter_pos - killer_pos).truncate().normalize_or_zero();
                    let angle = direction.y.atan2(direction.x);
                    let snapped_angle_90 = (angle / RAD_90).round() * RAD_90;
                    let snapped_angle_45 = (((angle - RAD_45) / RAD_90).round() * RAD_90) + RAD_45;
                    (snapped_angle_90 - RAD_90, snapped_angle_45 - RAD_90)
                    
                } else { (0.0, 0.0) }
            },
            None => (0.0, 0.0),
        };

        let mut rng = rand::thread_rng();
        
        //let colour = Color::hsl(rng.gen_range(0.0..360.0), 0.5, 0.5).to_linear();
        let colour = Color::hsl(0.0, 0.9, 0.3).to_linear();
        let rgb = Vec3::new(colour.red, colour.green, colour.blue);

        build_splat(&mut commands, &splat_db, &mut materials, &mut meshes, &time, splat_rot_90, transform.translation(), SplatType::Radial, rgb);
        build_splat(&mut commands, &splat_db, &mut materials, &mut meshes, &time, splat_rot_90, transform.translation(), SplatType::Long, rgb);
        build_splat(&mut commands, &splat_db, &mut materials, &mut meshes, &time, splat_rot_45 + RAD_45, transform.translation(), SplatType::Diagonal, rgb);


    }
}

pub fn build_splat(
    commands: &mut Commands, 
    splat_db: &Res<SplatDb>, 
    materials: &mut ResMut<Assets<SplatMaterial>>, 
    meshes: &mut ResMut<Assets<Mesh>>, 
    time: &Res<Time>, 
    splat_rotation: f32, 
    pos: Vec3,
    splat_type: SplatType,
    colour: Vec3,
) {
    let adjusted_rotation = Quat::from_rotation_z(splat_rotation);
    let Some((splat_tex, splat_rect, origin_offset)) = splat_db.random_of_type(splat_type) else { return };

    let uv_rect = Vec4::new(splat_rect.min.x / 1024.0, splat_rect.min.y / 1024.0, splat_rect.max.x / 1024.0, splat_rect.max.y / 1024.0);

    let splat_mat = materials.add(SplatMaterial {
        current_time: time.elapsed_secs(),
        texture: splat_tex,
        uv_rect: uv_rect,
        colour
    });
    let mesh = meshes.add(Mesh::from(Rectangle::default()));
    let translation = pos - (adjusted_rotation * origin_offset.extend(0.0));
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(splat_mat),
        Transform { 
            translation: translation.with_z(-10.0), 
            scale: Vec3::new(splat_rect.width(), splat_rect.height(), 1.0),
            rotation: adjusted_rotation,
            ..default()
        },
        Splat,
        StageObject::StagePersistent
    ));
}