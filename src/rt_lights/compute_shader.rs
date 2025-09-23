
use bevy::prelude::*;
use bevy_app_compute::prelude::*;


#[derive(TypePath)]
struct RTLComputeShader;

impl ComputeShader for RTLComputeShader {
    fn shader() -> ShaderRef {
        "shaders/rtl/rtl_compute.wgsl".into()
    }
}



#[derive(Resource)]
pub(crate) struct RTLComputeWorker;

impl ComputeWorker for RTLComputeWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {
        let worker = AppComputeWorkerBuilder::new(world)
            .add_uniform("uni", &5.0)
            //.add_storage("shared_buffer_name", &[0f32; 4])
            .add_pass::<RTLComputeShader>([4, 1, 1], &["uni"])
            .build();

        worker
    }
}

