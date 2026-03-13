
use bevy::prelude::*;

use crate::lit_sprite::{components::LitSpriteDb, global_components::LitSprite};

pub fn init_default_lit_sprite(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(LitSpriteDb {
        mesh_handle: meshes.add(Rectangle::default()),
        material_cache: default()
    });
}

pub fn handle_new_lit_sprites(
    mut commands: Commands,
    query: Query<(Entity, &LitSprite), Added<LitSprite>>,
    mut sprite_db: ResMut<LitSpriteDb>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, sprite) in query.iter() {

        let material_key = (sprite.albedo_texture.id(), sprite.specular_texture.id());
        let material_handle = sprite_db.material_cache.entry(material_key).or_insert(
            materials.add(ColorMaterial {
                texture: Some(sprite.albedo_texture.clone()),
                ..default()
            })
        ).clone();

        commands.entity(entity).insert((
            Mesh2d(sprite_db.mesh_handle.clone()),
            MeshMaterial2d(material_handle)
        ));
    }
}