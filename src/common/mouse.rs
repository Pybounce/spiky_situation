
use bevy::{prelude::*, window::PrimaryWindow};


#[derive(Resource, Default)]
pub struct MouseData {
    pub world_position: Vec2,
    pub window_position_normalised: Vec2
}


pub fn update_mouse_data(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut mouse_data: ResMut<MouseData>,

) {
    let (camera, camera_transform) = q_camera.single().unwrap();
    let window = q_window.single().unwrap();

    if let Some(window_position) = window.cursor_position() {
        mouse_data.window_position_normalised = Vec2::new(window_position.x / window.width(), window_position.y / window.height());
        
    }

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        mouse_data.world_position = world_position;
    }
}