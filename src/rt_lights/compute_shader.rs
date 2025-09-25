
use bevy::{prelude::*, render::{render_resource::Buffer, Extract}};
use bevy_app_compute::prelude::*;
use bytemuck::{Pod, Zeroable};

use crate::{ground::Ground, rt_lights::components::PointLight, stage::stage_objects::pressure_spikes::PressureSpike};

const MAX_LIGHTS: u32 = 30;

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

#[repr(C)]
#[derive(Default, Clone, Copy, ShaderType, Pod, Zeroable)]
pub struct RTPointLight {
    pub pos: Vec2,
    pub packed_light: u32,
    pub _pad: u32
}

impl RTPointLight {
    pub fn new(pos: Vec2, colour: Color, intensity: u8) -> Self {
        let [r, g, b] = colour.to_linear().to_u8_array_no_alpha();
        let mut packed_light: u32 = 0;
        packed_light |= (r as u32) << 24;
        packed_light |= (g as u32) << 16;
        packed_light |= (b as u32) << 8;
        packed_light |= intensity as u32;
        
        return Self {
            packed_light,
            pos,
            _pad: 0,
        };
    }
}

#[derive(Resource)]
pub(crate) struct RTLComputeWorker;

impl ComputeWorker for RTLComputeWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {


        let rays_per_light = 320;
        let ray_workgroup_size = 64;
        let ray_workgroup_count = (rays_per_light + ray_workgroup_size - 1) / ray_workgroup_size;
        

        let worker = AppComputeWorkerBuilder::new(world)
            .add_storage("lighting_output", &[0u32; 1600*1600])
            .add_storage("occluder_mask", &[0u32; 1600*1600])
            .add_storage("lights", &[RTPointLight::default(); MAX_LIGHTS as usize])
            .add_uniform("light_count", &0)
            .add_pass::<RTLResetShader>([100, 100, 1], &["lighting_output"])
            .add_pass::<RTLComputeShader>([ray_workgroup_count, MAX_LIGHTS, 1], &["light_count", "lights", "lighting_output", "occluder_mask"])
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



#[derive(Resource)]
pub(crate) struct OccluderMask(pub Vec<u32>);

pub fn init_occluder_mask(
    mut commands: Commands
) {
    commands.insert_resource(OccluderMask(vec![0u32; 1600*1600]));
}

/// TODO: Can have an UpdateEvent or I guess just track change diffs myself for this. (will need updates on occluder layout change whether that's mid level from dynamic ones or on new level loaded)
pub(crate) fn write_occluder_buffer(
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

pub(crate) fn update_rt_lights(
    query: Query<(&Transform, &PointLight)>,
    mut worker: ResMut<AppComputeWorker<RTLComputeWorker>>,
) {
    let mut current_count = 0u32;
    let mut lights: Vec<RTPointLight> = vec![];

    for (transform, light) in query {
        lights.push(RTPointLight::new(transform.translation.truncate(), light.colour, light.intensity));
        current_count += 1;
        if current_count >= MAX_LIGHTS {
            break;
        }
    }

    worker.write_slice("lights", &lights);
    worker.write("light_count", &current_count);
}


