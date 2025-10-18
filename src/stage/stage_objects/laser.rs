use std::f32;

use bevy::{math::VectorSpace, prelude::*};
use avian2d::prelude::*;

use crate::{common::{animated_sprite::SpriteAnimator, physics::layers::GamePhysicsLayer, rails::RailRider}, ground::Ground, obstacles::InstantKiller, rt_lights::components::{AreaLight, LightOccluder}, stage::{stage_builder::{stage_asset, stage_creator::{get_object_tilemap_rect_from_index, ObjectAtlasIndices, StageCreator, TILE_SIZE, TILE_SIZE_HALF}}, stage_objects::{tiles::PhysicalTileBundle, StageObject}}};


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
            RigidBody::Static,
            Collider::rectangle(TILE_SIZE_HALF, 1.0),
            CollisionEventsEnabled,
            Sensor,
            InstantKiller,
            StageObject::Volatile,
            CollisionLayers::new(GamePhysicsLayer::StageObject, LayerMask::ALL),
            AreaLight {
                intensity: 6.0,
                colour: Color::srgb_u8(255, 0, 0),
                rect: Rect::new(16.0, 16.0, 16.0, 16.0),
            }
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

        let mut laser_core = commands.spawn((
            PhysicalTileBundle::new(stage_creator, laser.grid_pos, atlas_rects[0], laser.rotation, stage_creator.object_tilemap, CollisionLayers::new(GamePhysicsLayer::Ground, LayerMask::ALL)),
            SpriteAnimator::new_non_repeating(50, atlas_rects),
            Laser {
                beam,
                beam_end_particles
            },
            Ground,
            RayCaster::new(Vec2::Y * 8.1, Dir2::Y).with_ignore_self(true).with_solidness(true).with_query_filter(SpatialQueryFilter::from_mask(GamePhysicsLayer::Ground)),
            LightOccluder::Rect(16.0, 16.0)
        ));

        if let Some(rail_rider) = &laser.rail_rider {
            laser_core.insert(RailRider {
                rail_id: rail_rider.rail_id,
                current_waypoint_index: rail_rider.next_waypoint,
                reversed: rail_rider.reversed,
            });
        }   
    }

}

pub fn update_laser_beams(
    laser_query: Query<(&Transform, &Laser, &RayCaster, &RayHits), (Without<LaserBeam>, Without<LaserBeamEndParticles>)>,
    mut beam_query: Query<(&mut Transform, &mut AreaLight), With<LaserBeam>>,
    mut end_particles_query: Query<&mut Transform, (With<LaserBeamEndParticles>, Without<LaserBeam>, Without<Laser>)>

) {
    for (laser_transform, laser, ray, ray_hits) in &laser_query {
        let mut current_min_dist = f32::MAX;
        for hit in ray_hits.iter() {

            if hit.distance < current_min_dist { current_min_dist = hit.distance; } else { continue; }

            let ray_origin = laser_transform.translation.truncate() + (laser_transform.rotation * ray.origin.extend(0.0)).truncate();

            let hit_point = ray_origin + (*ray.global_direction() * hit.distance);

            if let Ok((mut beam_transform, mut area_light)) = beam_query.get_mut(laser.beam) {
                beam_transform.translation = (ray_origin + ((hit_point - ray_origin) / 2.0)).extend(90.0);
                beam_transform.scale.y = hit.distance + 2.0;
                beam_transform.rotation = laser_transform.rotation;

                area_light.rect = Rect::new(0.0, 0.0, beam_transform.scale.y, 16.0);
                area_light.rect = Rect::new(0.0, 0.0, 16.0, beam_transform.scale.y);

            }

            // particles
            if let Ok(mut end_particles_transform) = end_particles_query.get_mut(laser.beam_end_particles) {
                end_particles_transform.translation = hit_point.extend(100.0);
                end_particles_transform.rotation = laser_transform.rotation;
            }
        }
    }
}
