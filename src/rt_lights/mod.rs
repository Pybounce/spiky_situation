
use bevy::{prelude::*, render::RenderApp};
use bevy_app_compute::prelude::*;

use crate::rt_lights::{compute_shader::{extract_lighting_out_buffer, increment_temporal_lightmap_index, init_occluder_mask, refresh_static_occluders, update_occluder_current_frame, update_rt_lights, write_occluder_buffer, RTLComputeWorker, TemporalLightmapIndex}, occluders::setup_occluder_map, post_process_shader::RTLPostProcessPlugin};

pub(crate) mod compute_shader;
pub(crate) mod post_process_shader;
pub mod components;
mod occluders;

pub struct RTLightPlugin;




impl Plugin for RTLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AppComputePlugin)
        .add_plugins(AppComputeWorkerPlugin::<RTLComputeWorker>::default())
        .add_plugins(RTLPostProcessPlugin)
        .init_resource::<TemporalLightmapIndex>()
        .add_systems(Startup, (init_occluder_mask).chain())
        .add_systems(PreUpdate, (update_occluder_current_frame, refresh_static_occluders, increment_temporal_lightmap_index))
        .add_systems(Update, (write_occluder_buffer, update_rt_lights));

    app.sub_app_mut(RenderApp)
       .add_systems(ExtractSchedule, extract_lighting_out_buffer);
    }
}