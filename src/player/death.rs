
use bevy::prelude::*;

use crate::{builders::player_builders::PlayerBuilder, common::death::DeathMarker, databases::splat_db::SplatDb, local_player::LocalPlayer, shaders::splat::SplatMaterial};


#[derive(Component)]
pub struct Respawnable {
    pub translation: Vec3,
    pub delay_in_seconds: f64
}

pub fn spawn_player_corpse(
    mut commands: Commands,
    query: Query<&Transform, (With<LocalPlayer>, Added<DeathMarker>)>,
    player_builder: Res<PlayerBuilder>,
) {
    for transform in &query {
        let mut corpse = commands.spawn(());
        player_builder.build_player_corpse(&mut corpse, transform.translation + Vec3::new(0.0, 0.0, 100.0));
    }
}

pub fn player_splat(
    mut commands: Commands,
    query: Query<&Transform, (With<LocalPlayer>, Added<DeathMarker>)>,
    mut materials: ResMut<Assets<SplatMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    splat_db: Res<SplatDb>
) {
    for transform in &query {
        let (splat_tex, splat_rect) = splat_db.random_radial();

        let splat_mat = materials.add(SplatMaterial {
            current_time: time.elapsed_secs(),
            texture: splat_tex,
            uv_rect: Vec4::new(splat_rect.min.x, splat_rect.min.y, splat_rect.max.x, splat_rect.max.y),
        });
        let mesh = meshes.add(Mesh::from(Rectangle::default()));

        commands.spawn((
            Mesh2d(mesh),
            MeshMaterial2d(splat_mat),
            Transform { 
                translation: transform.translation - (Vec3::Z * 10.0), 
                scale: Vec3::splat(64.0),
                ..default()
            },
        ));

    }
}

