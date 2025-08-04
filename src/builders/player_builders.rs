use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier2d::prelude::{RigidBody, Velocity};

use crate::{common::{death::DelayedDeathMarker, physics::gravity::Gravity}, local_player::PLAYER_SIZE, stage::stage_builder::stage_creator::TILE_SIZE};

#[derive(Resource)]
pub struct PlayerBuilder {
    player_atlas: Handle<Image>
}

impl PlayerBuilder {
    pub fn build_player_corpse(&self, entity_commands: &mut EntityCommands, pos: Vec3) {
        let player_corpse_rect = Rect::new(TILE_SIZE * 1.0, TILE_SIZE, TILE_SIZE * 2.0, TILE_SIZE * 2.0);

        entity_commands.try_insert((
            Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                rect: Some(player_corpse_rect),
                image: self.player_atlas.clone(),
                ..default()
            },
            Transform::from_scale(PLAYER_SIZE.extend(1.0)).with_translation(pos),
            DelayedDeathMarker::from_secs(5.0),
            RigidBody::Dynamic,
            Velocity::linear(Vec2::new(0.0, 200.0)),
            Gravity {
                max_force: 400.0,
                current_force: 0.0,
                acceleration: 3000.0,
            }
        ));
    }
}


pub fn init_player_builder(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

) {
    let tilemap: Handle<Image> = asset_server.load("object_tilemap.png");
    commands.insert_resource(PlayerBuilder {
        player_atlas: tilemap
    });
}