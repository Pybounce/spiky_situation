
use bevy::{prelude::*, render::{render_resource::Buffer, Extract}};
use bevy_app_compute::prelude::*;

use crate::ground::Ground;


#[derive(TypePath)]
struct RTLComputeShader;

#[derive(TypePath)]
struct RTLResetShader;

impl ComputeShader for RTLComputeShader {
    fn shader() -> ShaderRef {
        "shaders/rtl/rtl_compute.wgsl".into()
    }
}
impl ComputeShader for RTLResetShader {
    fn shader() -> ShaderRef {
        "shaders/rtl/rtl_reset.wgsl".into()
    }
}


#[derive(Resource)]
pub(crate) struct RTLComputeWorker;

impl ComputeWorker for RTLComputeWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {
        let light_count = 1;
        let rays_per_light = 360;
        let total_rays = light_count * rays_per_light;
        let workgroup_size = 64;
        let workgroup_count = (total_rays + workgroup_size - 1) / workgroup_size;
        

        let worker = AppComputeWorkerBuilder::new(world)
            .add_uniform("uni", &5.0)
            .add_storage("lighting_output", &[0u32; 1600*1600])
            .add_storage("occluder_mask", &[0u32; 1600*1600])
            .add_pass::<RTLResetShader>([100, 100, 1], &["uni", "lighting_output"])
            .add_pass::<RTLComputeShader>([workgroup_count, 1, 1], &["uni", "lighting_output", "occluder_mask"])
            .build();

            worker
    }
}


#[derive(Resource)]
pub(crate) struct SharedRTLOutputBuffer(pub Buffer);

pub(crate) fn extract_lighting_out_buffer(
    mut commands: Commands,
    worker: Extract<Res<AppComputeWorker<RTLComputeWorker>>>
) {
    if let Some(buf) = worker.get_buffer("lighting_output") {
        commands.insert_resource(SharedRTLOutputBuffer(buf.clone()));
    }
    
}

//pub enum LightOccluder {
//    Box(f32, f32)
//}
//
//pub struct PointLight {
//    pub intensity: f32,
//    pub colour: Color
//}

#[derive(Resource)]
pub(crate) struct OccluderMask(pub Vec<u32>);

pub fn init_occluder_mask(
    mut commands: Commands
) {
    commands.insert_resource(OccluderMask(vec![0u32; 1600*1600]));
}

/// TODO: Can have an UpdateEvent or I guess just track change diffs myself for this. (will need updates on occluder layout change whether that's mid level from dynamic ones or on new level loaded)
pub fn write_occluder_buffer(
    query: Query<&Transform, With<Ground>>,
    mut worker: ResMut<AppComputeWorker<RTLComputeWorker>>,
    mut mask: ResMut<OccluderMask>,
    input: Res<ButtonInput<KeyCode>>,

) {

    if input.just_pressed(KeyCode::KeyL) {
        mask.0.fill(0);

        for t in query.iter() {
            let gx = t.translation.x as i32;
            let gy = t.translation.y as i32;
            for y in 0..16 {
                for x in 0..16 {
                    let px = gx + x;
                    let py = gy + y;

                    if px >= 0 && px < 1600 && py >= 0 && py < 1600 {
                        mask.0[(py as usize) * 1600 + (px as usize)] = 1;
                    }
                }
            }
        }

        worker.write_slice("occluder_mask", &mask.0);
    }


}