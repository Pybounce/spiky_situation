
use bevy::{prelude::*, render::{camera::RenderTarget, render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, view::{Layer, RenderLayers}}};

#[derive(Resource)]
pub struct OccluderMap(pub Handle<Image>);

const OCCLUDER_LAYER: Layer = 1;

pub fn setup_occluder_map(
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands
) {
    let width = 1600.0;
    let height = 1600.0;

    let occluder_texture = Image {
        texture_descriptor: bevy::render::render_resource::TextureDescriptor {
            size: Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: 1,
            },
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::STORAGE_BINDING,
            label: Some("OcclusionRenderTarget".into()),
            view_formats: &[],
        },
        ..Default::default()
    };
    let occluder_texture_handle = images.add(occluder_texture);

    commands.insert_resource(OccluderMap(occluder_texture_handle.clone()));



    commands.spawn((
        Camera {
            order: 1,
            target: RenderTarget::Image(occluder_texture_handle.into()),
            ..default()
        },
        Camera2d,
        GlobalTransform::from_translation(Vec3::new(width / 2.0, height / 2.0, 0.0)),
        Transform::default(),
        Projection::Orthographic(OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: 1.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            scaling_mode: bevy::render::camera::ScalingMode::Fixed { width, height },
            area: Default::default(),
        }),
        RenderLayers::layer(OCCLUDER_LAYER)
    ));

}
