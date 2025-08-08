use bevy::prelude::*;

use crate::builders::player_builders::PlayerBuilder;


#[derive(Component)]
pub struct LocalPlayerSpawner {
    pub spawn_time: f64,
    pub translation: Vec3
}

pub fn spawn_local_players(
    query: Query<(Entity, &LocalPlayerSpawner)>,
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>
) {
    for (entity, spawner) in &query {
        if time.elapsed_secs_f64() >= spawner.spawn_time {
            let mut player = commands.spawn(());
            PlayerBuilder::build_player(&mut player, &asset_server, spawner.translation);
            commands.entity(entity).despawn();
        }
    }

}