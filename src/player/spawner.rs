use bevy::prelude::*;

use crate::{local_player::LocalPlayerBundle, stage::stage_builder::CurrentStageData};


#[derive(Component)]
pub struct LocalPlayerSpawner {
    pub spawn_time: f64,
    pub translation: Vec3
}

pub fn spawn_local_players(
    query: Query<(Entity, &LocalPlayerSpawner)>,
    stage_data_opt: Option<Res<CurrentStageData>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if let Some(stage_data) = stage_data_opt {
        for (entity, spawner) in &query {
            if time.elapsed_secs_f64() >= spawner.spawn_time {
                commands.spawn(LocalPlayerBundle::new(spawner.translation, stage_data.stage_id));
                commands.entity(entity).despawn();
            }
        }
    }

}