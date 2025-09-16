
use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};


#[derive(Resource, Default)]
pub struct MouseData {
    pub world_position: Vec2,
    pub window_position_normalised: Vec2
}

#[derive(Event)]
pub struct WorldMouseMotion {
    pub delta: Vec2
}


pub fn update_mouse_data(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut mouse_data: ResMut<MouseData>,
    mut world_mouse_motion_writer: EventWriter<WorldMouseMotion>
) {
    let (camera, camera_transform) = q_camera.single().unwrap();
    let window = q_window.single().unwrap();

    if let Some(window_position) = window.cursor_position() {
        
        let old_window_position = mouse_data.window_position_normalised * Vec2::new(window.width(), window.height());
        
        mouse_data.window_position_normalised = Vec2::new(window_position.x / window.width(), window_position.y / window.height());
        
        let start = camera.viewport_to_world(camera_transform, old_window_position).unwrap().origin.truncate();
        let end = camera.viewport_to_world(camera_transform, window_position).unwrap().origin.truncate();
        world_mouse_motion_writer.write(WorldMouseMotion {
            delta: end - start
        });
    }

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        mouse_data.world_position = world_position;
    }
}