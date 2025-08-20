use bevy::prelude::*;

use crate::shaders::cctv_shader::plugin::CCTVPostProcessSettings;

pub mod plugin;

pub fn update_cctv_shader_time(
    mut query: Query<&mut CCTVPostProcessSettings>,
    time: Res<Time>
) {
    for mut settings in &mut query {
        settings.time = time.elapsed_secs();
    }
}