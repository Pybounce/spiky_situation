
use bevy::{prelude::*, render::RenderApp};
use bevy_app_compute::prelude::*;

use crate::rt_lights::{compute_shader::{extract_lighting_out_buffer, init_occluder_mask, write_occluder_buffer, RTLComputeWorker}, post_process_shader::RTLPostProcessPlugin};

pub(crate) mod compute_shader;
pub(crate) mod post_process_shader;

pub struct RTLightPlugin;




impl Plugin for RTLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AppComputePlugin)
        .add_plugins(AppComputeWorkerPlugin::<RTLComputeWorker>::default())
        .add_plugins(RTLPostProcessPlugin)
        .add_systems(Startup, (init_occluder_mask).chain())
        .add_systems(PostUpdate, write_occluder_buffer);

    app.sub_app_mut(RenderApp)
       .add_systems(ExtractSchedule, extract_lighting_out_buffer);
    }
}