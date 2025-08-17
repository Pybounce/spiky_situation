
use bevy::{core_pipeline::bloom::Bloom, input::mouse::MouseWheel, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{local_player::LocalPlayer, shaders::cctv_shader::plugin::CCTVPostProcessSettings};

const CAMERA_ZOOM: u32 = 3;
const CAMERA_ZOOM_MAX: u32 = 10;
const CAMERA_ZOOM_MIN: u32 = 1;

pub fn spawn_camera(mut commands: Commands) {

    commands
        .spawn((
            Camera2d::default(),
            Projection::Orthographic(OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scale: 1.0 / (CAMERA_ZOOM as f32),
                viewport_origin: Vec2::new(0.5, 0.5),
                scaling_mode: bevy::render::camera::ScalingMode::WindowSize,
                area: Default::default(),
            }),
            Transform::default(),
            Velocity::default(),
            RigidBody::Dynamic,
            PixelPerfectTranslation {
                translation: Vec3::default(),
                factor: CAMERA_ZOOM as u32
            },
            Msaa::Off,
            CCTVPostProcessSettings {
                time: 0.0,
                chromatic_intensity: 0.0005,
                fisheye_intensity: 0.025,
                vignette_intensity: 0.4,
                vignette_start: 70.0,
                scanline_dark_mul: 0.95,
                scanline_width: 0.15,
                scanline_speed: 0.7,
                scanline_gap: 7.0,
            },
        ));
}

pub fn move_camera(
    mut camera_query: Query<&mut PixelPerfectTranslation, With<Camera>>,
    player_query: Query<&Transform, (With<LocalPlayer>, Without<Camera>)>,
    time: Res<Time>
) {
    let mut ct = camera_query.single_mut().unwrap();
    let pt = player_query.single();
    match pt {
        Ok(pt) => {
            let distance = ct.translation.truncate().distance(pt.translation.truncate());
            let speed = distance.powf(1.1) * 2.5;
            let dir = (pt.translation - ct.translation).truncate().normalize_or_zero();

            let delta = time.delta_secs() * speed * dir;
            ct.translation += delta.extend(0.0);
        }
        Err(_) => (),
    }
}

pub fn move_pixel_perfect_translations(
    mut query : Query<(&mut Transform, &PixelPerfectTranslation)>,
) {
    for (mut t, pp) in &mut query {
        t.translation = Vec3::new(
            round_by_factor(pp.translation.x, pp.factor), 
            round_by_factor(pp.translation.y, pp.factor), 
            round_by_factor(pp.translation.z, pp.factor)); 
    }
}

fn round_by_factor(val: f32, factor: u32) -> f32 {
    (val * factor as f32).trunc() / factor as f32
}

#[derive(Component)]
pub struct PixelPerfectTranslation {
    pub translation: Vec3,
    pub factor: u32
}



pub fn handle_zoom_change(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut PixelPerfectTranslation, &mut Projection), With<Camera>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut pixel_translation, mut projection) in &mut camera_query {

            let new_zoom = match mouse_wheel_event.y > 0.0 {
                true => pixel_translation.factor + 1,
                false => pixel_translation.factor - 1,
            }.clamp(CAMERA_ZOOM_MIN, CAMERA_ZOOM_MAX);

            if let Projection::Orthographic(ref mut ortho) = *projection {
                ortho.scale = 1.0 / (new_zoom as f32);
            };
            pixel_translation.factor = new_zoom;
        }
    }
}