
use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}, window::WindowResized};

use crate::lit_sprite::{components::{DefaultSpecularTexture, LitSpriteDb, LitSpriteMaterialKey, LitSpriteMeshKey}, global_components::{LitSprite, LitSpriteMaterial, SpecularBuffer}};

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
    buffer: Res<SpecularBuffer>,
    default_specular: Res<DefaultSpecularTexture>
) {
    for (entity, sprite) in query.iter() {
        
        let uv_rect = sprite.flipped_rect().unwrap_or(Vec4::new(0.0, 0.0, 1.0, 1.0));
        let material_key = LitSpriteMaterialKey::from_sprite(sprite);
        let material_handle = sprite_db.material_cache.entry(material_key).or_insert(
            materials.add(LitSpriteMaterial {
                albedo_texture: sprite.albedo_texture.clone(),
                specular_texture: sprite.specular_texture.clone().unwrap_or_else(|| default_specular.0.clone()).into(),
                uv_rect: uv_rect,
                specular_output: buffer.handle.clone().into()
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

pub fn init_default_specular(
    mut commands: Commands, 
    mut images: ResMut<Assets<Image>>
) {
    let black_image = Image::new_fill(
        Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &[127, 127, 127, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    commands.insert_resource(DefaultSpecularTexture(images.add(black_image)));
}

pub fn init_specular_buffer(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else { return; };
    
    let handle = create_screen_texture(&mut images, window.resolution.physical_width(), window.resolution.physical_height());
    commands.insert_resource(SpecularBuffer { handle });
}

pub fn resize_specular_buffer(
    mut resize_events: EventReader<WindowResized>,
    windows: Query<&Window>,
    mut images: ResMut<Assets<Image>>,
    mut buffer: ResMut<SpecularBuffer>,
) {
    if !resize_events.is_empty() {
        resize_events.clear();
        
        let Ok(window) = windows.single() else { return; };
        let width = window.resolution.physical_width();
        let height = window.resolution.physical_height();
        
        buffer.handle = create_screen_texture(&mut images, width, height);
    }
}

pub fn update_materials_with_buffer(
    buffer: Res<SpecularBuffer>,
    mut materials: ResMut<Assets<LitSpriteMaterial>>,
) {
    if buffer.is_changed() {
        for (_, mat) in materials.iter_mut() {
            mat.specular_output = Some(buffer.handle.clone());
        }
    }
}

fn create_screen_texture(images: &mut Assets<Image>, width: u32, height: u32) -> Handle<Image> {
    let size = Extent3d {
        width: width.max(1),
        height: height.max(1),
        depth_or_array_layers: 1,
    };

    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    
    image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::STORAGE_BINDING;
    
    images.add(image)
}