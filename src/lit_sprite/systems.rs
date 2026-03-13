
use bevy::prelude::*;

use crate::lit_sprite::{components::{LitSpriteDb, LitSpriteMaterialKey}, global_components::{LitSprite, LitSpriteMaterial}};

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
    mut materials: ResMut<Assets<LitSpriteMaterial>>,
) {
    for (entity, sprite) in query.iter() {

        let rect = sprite.rect.unwrap_or(Rect::new(0.0, 0.0, 1.0, 1.0));
        let material_key = LitSpriteMaterialKey {
            albedo_id: sprite.albedo_texture.as_ref().map(|h| h.id()),
            specular_id: sprite.specular_texture.as_ref().map(|h| h.id()),
            rect_bits: [
                rect.min.x.to_bits(),
                rect.min.y.to_bits(),
                rect.width().to_bits(),
                rect.height().to_bits(),
            ]
        };
        let material_handle = sprite_db.material_cache.entry(material_key).or_insert(
            materials.add(LitSpriteMaterial {
                albedo_texture: sprite.albedo_texture.clone(),
                specular_texture: None,
                uv_rect: Vec4::new(rect.min.x, rect.min.y, rect.width(), rect.height())
            })
        ).clone();

        commands.entity(entity).insert((
            Mesh2d(sprite_db.mesh_handle.clone()),
            MeshMaterial2d(material_handle)
        ));
    }
}