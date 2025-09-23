
use bevy::prelude::*;
use bevy_app_compute::prelude::*;

use crate::rt_lights::compute_shader::RTLComputeWorker;

pub(crate) mod compute_shader;
pub(crate) mod post_process_shader;

pub struct RTLightPlugin;




impl Plugin for RTLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AppComputePlugin)
        .add_plugins(AppComputeWorkerPlugin::<RTLComputeWorker>::default());
    }
}