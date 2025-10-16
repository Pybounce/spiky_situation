
use bevy::{core_pipeline::{bloom::Bloom, tonemapping::Tonemapping}, input::mouse::MouseWheel, prelude::*, window::WindowResized};
use avian2d::prelude::*;

use crate::{local_player::LocalPlayer, rt_lights::post_process_shader::RTLPostProcessSettings, shaders::cctv_shader::plugin::CCTVPostProcessSettings, stage::stage_builder::CurrentStageData};

const CAMERA_ZOOM: u32 = 3;
const CAMERA_ZOOM_MAX: u32 = 10;
const CAMERA_ZOOM_MIN: u32 = 1;

pub fn spawn_camera(mut commands: Commands) {

    commands
        .spawn((
            Camera2d::default(),
            Camera {
                //hdr: true,
                ..default()
            },
            Projection::Orthographic(OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scale: 1.0 / (CAMERA_ZOOM as f32),
                viewport_origin: Vec2::new(0.5, 0.5),
                scaling_mode: bevy::render::camera::ScalingMode::WindowSize,
                area: Default::default(),
            }),
            Transform::default(),
            //LinearVelocity::default(),
            //RigidBody::Dynamic,
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
            RTLPostProcessSettings {
                something: 1.0
            },
            //Bloom::NATURAL
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

            let delta = (time.delta_secs() * speed).min(distance) * dir;
            
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



pub fn clamp_window_resolution(
    mut resize_events: EventReader<WindowResized>,
    mut windows: Query<&mut Window>,
) {
    for event in resize_events.read() {
        if let Ok(mut window) = windows.single_mut() {
            let tile_size = 16.0;
            let new_width = (event.width / tile_size).floor() * tile_size;
            let new_height = (event.height / tile_size).floor() * tile_size;

            window.resolution.set(new_width, new_height);
        }
    }
}

const MIN_STAGE_SIZE: Vec2 = Vec2::new(16.0 * 32.0, 16.0 * 18.0);


pub fn clamp_camera_to_stage(
    windows: Query<&Window>,
    mut camera_query: Query<(&mut PixelPerfectTranslation, &mut Transform, &mut Projection), With<Camera>>,
    stage_data: Option<Res<CurrentStageData>>,
) {

    let Ok(window) = windows.single() else { return ; };
    let window_res = Vec2::new(window.resolution.width(), window.resolution.height());
    let Some(stage_data) = stage_data else { return; };
    
    let max_scale = (window_res.x / MIN_STAGE_SIZE.x).max(window_res.y / MIN_STAGE_SIZE.y);
    let min_pixel_factor = max_scale.ceil().max(1.0) as u32;
    
    let scale = 1.0 / (min_pixel_factor as f32);
    
    //println!("Camera sees: {} x {}", view_size.x, view_size.y);
    //println!("scale: {}", 1.0 / pixel_factor as f32);
    //println!("max_scale: {}", max_scale);


    for (mut pixel_translation, mut transform, mut projection) in &mut camera_query {
        
        if let Projection::Orthographic(ref mut ortho) = *projection {
            let pixel_factor = min_pixel_factor.max(pixel_translation.factor);
            ortho.scale = 1.0 / pixel_factor as f32;
            pixel_translation.factor = pixel_factor;
        };

        let view_size = window_res * (1.0 / pixel_translation.factor as f32);
        pixel_translation.translation = pixel_translation.translation.clamp(Vec3::new(view_size.x / 2.0, view_size.y / 2.0, -10000.0), Vec3::new(stage_data.bounds.width() - (view_size.x / 2.0) - 16.0 , stage_data.bounds.height() - (view_size.y / 2.0) - 16.0, 10000.0));
    }
}

// THY CLAMPING PLAN

// Make a MIN_STAGE_SIZE
// Always clamp the scale such that scale * res is smaller than the MIN_STAGE_SIZE
// Always clamp the camera position such that the edges don't surpass x/y axis and MIN_STAGE_SIZE


// TODO 

// Let player zoom in/out within bounds
// Change some existing stages to fit camera

// HK and C both manage to get it working and looking decent so.