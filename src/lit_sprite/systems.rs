
use bevy::prelude::*;

use crate::lit_sprite::{components::{LitSpriteDb, LitSpriteMaterialKey, LitSpriteMeshKey}, global_components::{LitSprite, LitSpriteMaterial}};

pub fn init_default_lit_sprite(
    mut commands: Commands,
) {
    commands.insert_resource(LitSpriteDb {
        material_cache: default(),
        mesh_cache: default()
    });
}

pub fn handle_new_lit_sprites(
    mut commands: Commands,
    query: Query<(Entity, &LitSprite), Changed<LitSprite>>,
    mut sprite_db: ResMut<LitSpriteDb>,
    mut materials: ResMut<Assets<LitSpriteMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, sprite) in query.iter() {
        
        let uv_rect = sprite.flipped_rect().unwrap_or(Vec4::new(0.0, 0.0, 1.0, 1.0));
        let material_key = LitSpriteMaterialKey::from_sprite(sprite);
        let material_handle = sprite_db.material_cache.entry(material_key).or_insert(
            materials.add(LitSpriteMaterial {
                albedo_texture: sprite.albedo_texture.clone(),
                specular_texture: None,
                uv_rect: uv_rect
            })
        ).clone();
        
        let mesh_key = LitSpriteMeshKey::from_sprite(sprite);
        let mesh_handle = sprite_db.mesh_cache.entry(mesh_key).or_insert(
            meshes.add(Rectangle::from_size(sprite.size))
        ).clone();

        commands.entity(entity).insert((
            Mesh2d(mesh_handle),
            MeshMaterial2d(material_handle)
        ));
    }
}