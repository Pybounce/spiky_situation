use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{common::{animated_sprite::SpriteAnimator, physics::helpers::RapierRaycastExt}, ground::Ground, obstacles::InstantKiller, stage::{stage_builder::{stage_asset, stage_creator::{get_object_tilemap_rect_from_index, ObjectAtlasIndices, StageCreator, TILE_SIZE, TILE_SIZE_HALF}}, stage_objects::{tiles::PhysicalTileBundle, StageObject}}};


#[derive(Component)]
pub struct Laser {
    pub beam: Entity,
    pub beam_end_particles: Entity
}



#[derive(Component)]
pub struct LaserBeam;

#[derive(Component)]
pub struct LaserBeamEndParticles;

pub struct LaserBuilder;

impl LaserBuilder {
    pub fn spawn(commands: &mut Commands, stage_creator: &StageCreator, atlas_rects: Vec<Rect>, laser: &stage_asset::Laser) {
        let beam_atlas_rects = vec![
            get_object_tilemap_rect_from_index(ObjectAtlasIndices::Beam0),
            get_object_tilemap_rect_from_index(ObjectAtlasIndices::Beam1),
            get_object_tilemap_rect_from_index(ObjectAtlasIndices::Beam2),
            get_object_tilemap_rect_from_index(ObjectAtlasIndices::Beam3),
        ];
        let beam = commands.spawn((
            LaserBeam,
            Transform::default(),
            Sprite {
                image: stage_creator.object_tilemap.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, 1.0)),
                rect: Some(beam_atlas_rects[0]),
                ..default()
            },
            SpriteAnimator::new(200, beam_atlas_rects.clone()),
            RigidBody::Fixed,
            Collider::cuboid(TILE_SIZE_HALF / 2.0, 0.5),
            ActiveEvents::COLLISION_EVENTS,
            Sensor,
            InstantKiller,
            StageObject::Volatile,
            CollisionGroups::new(Group::GROUP_2, Group::ALL),
        )).id();

        let beam_end_particle_rects = vec![
            get_object_tilemap_rect_from_index(ObjectAtlasIndices::BeamEndParticles0),
            get_object_tilemap_rect_from_index(ObjectAtlasIndices::BeamEndParticles1),
            get_object_tilemap_rect_from_index(ObjectAtlasIndices::BeamEndParticles2),
            get_object_tilemap_rect_from_index(ObjectAtlasIndices::BeamEndParticles3),
        ];

        let beam_end_particles = commands.spawn((
            LaserBeamEndParticles,
            Transform::default(),
            Sprite {
                image: stage_creator.object_tilemap.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                rect: Some(beam_end_particle_rects[0]),
                ..default()
            },
            SpriteAnimator::new(100, beam_end_particle_rects.clone()),
            StageObject::Volatile
        )).id();

        commands.spawn((
            PhysicalTileBundle::new(stage_creator, laser.grid_pos, atlas_rects[0], laser.rotation, stage_creator.object_tilemap, CollisionGroups::new(Group::GROUP_1, Group::ALL)),
            SpriteAnimator::new_non_repeating(50, atlas_rects),
            Laser {
                beam,
                beam_end_particles
            },
            Ground,
        ));
    }

}

pub fn update_laser_beams(
    laser_query: Query<(&Transform, &Laser), (Without<LaserBeam>, Without<LaserBeamEndParticles>)>,
    rapier_write_context: WriteRapierContext,
    mut beam_query: Query<&mut Transform, With<LaserBeam>>,
    mut end_particles_query: Query<&mut Transform, (With<LaserBeamEndParticles>, Without<LaserBeam>, Without<Laser>)>

) {
    let rapier_ctx = rapier_write_context.single().unwrap();

    for (laser_transform, laser) in &laser_query {

        let origin = (laser_transform.translation + (laser_transform.rotation * (Vec3::Y * 8.1))).truncate();
        let dir = (laser_transform.rotation * Vec3::Y).truncate().normalize();

        if let Some((_entity, distance)) = rapier_ctx.raycast_group(origin, dir, 1000.0, Group::GROUP_1) {
            let hit_point = origin + (dir * distance);

            // beam
            if let Ok(mut beam_transform) = beam_query.get_mut(laser.beam) {
                beam_transform.translation = (origin + ((hit_point - origin) / 2.0)).extend(100.0);
                beam_transform.scale.y = distance + 2.0;
                beam_transform.rotation = laser_transform.rotation;
            }


            // particles
            if let Ok(mut end_particles_transform) = end_particles_query.get_mut(laser.beam_end_particles) {
                end_particles_transform.translation = hit_point.extend(100.0);
                end_particles_transform.rotation = laser_transform.rotation;
            }
        }

    }
}
