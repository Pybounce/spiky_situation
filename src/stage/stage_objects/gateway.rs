use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{common::physics::{avian_ex::ManyCollidingEntities, player_collision::PlayerCollisionEnded}, local_player::LocalPlayer, stage::{levels::data::CurrentLevelData, stage_builder::{events::BuildStageEvent, stage_asset, stage_creator::{StageCreator, TILE_SIZE, TILE_SIZE_HALF}, CurrentStageData}}};

#[derive(Component)]
pub struct Gateway(usize);

pub struct GatewayFactory;

impl GatewayFactory {
    pub fn spawn(commands: &mut Commands, gateway: &stage_asset::Gateway) {
        commands.spawn((
            Sensor,
            Collider::rectangle(TILE_SIZE, TILE_SIZE * 3.0),
            CollisionEventsEnabled,
            Transform {
                rotation: Quat::from_rotation_z(gateway.rotation),
                translation: Vec3::new((gateway.grid_pos.x * TILE_SIZE) + TILE_SIZE_HALF, (gateway.grid_pos.y * TILE_SIZE) + TILE_SIZE_HALF, 0.0),
                ..default()
            },
            Gateway(gateway.gateway_id)
        ));
    }
}

// all it needs to do is send out an event to load stage of the mapped id and mapped gateway

pub fn check_gateways(
    player_query: Query<&Transform, Without<Gateway>>,
    gateway_query: Query<(&Transform, &Gateway)>,
    mut player_collision_reader: EventReader<PlayerCollisionEnded>,
    current_level_data_opt: Option<Res<CurrentLevelData>>,
    current_stage_data_opt: Option<Res<CurrentStageData>>,
    mut build_event_writer: EventWriter<BuildStageEvent>,

) {
    let Some(current_level_data) = current_level_data_opt else { return; };
    let Some(current_stage_data) = current_stage_data_opt else { return; };

    let mut raised = false;
    for player_collision in player_collision_reader.read() {

        let Ok(player_transform) = player_query.get(player_collision.player) else { continue; };

        if let Ok((gateway_transform, gateway)) = gateway_query.get(player_collision.other) {
            let local_left = gateway_transform.rotation * Vec3::X;
            let to_player = player_transform.translation - gateway_transform.translation;
            if local_left.dot(to_player) < 0.0 && raised == false {
                let Some((new_stage, new_gateway)) = current_level_data.gateway_pairs.get(&(current_stage_data.stage_id, gateway.0)) else { continue; };
                build_event_writer.write(BuildStageEvent {stage_id: *new_stage, gateway_id_opt: Some(*new_gateway) });
                raised = true;
            }
        }
    }
}


