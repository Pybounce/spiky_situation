
use bevy::{prelude::*, render::{render_resource::Buffer, Extract}};
use bevy_app_compute::prelude::*;
use bytemuck::{Pod, Zeroable};

use crate::rt_lights::{components::{StaticLightOccluder, LightOccluder, PointLight}, occluders::OccluderMap};

const MAX_LIGHTS: u32 = 30;
const MAX_OCCLUDERS: u32 = 100*100;
const OCCLUDER_FRAME_BUDGET: u32 = 4;
const RESOLUTION: u32 = 1600;   // mostly unused at the moment

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

#[derive(TypePath)]
struct RTLOccludeFillShader;

impl ComputeShader for RTLOccludeFillShader {
    fn shader() -> ShaderRef {
        "shaders/rtl/rtl_occlude_fill.wgsl".into()
    }
}

#[derive(TypePath)]
struct RTLOccluderResetShader;

impl ComputeShader for RTLOccluderResetShader {
    fn shader() -> ShaderRef {
        "shaders/rtl/rtl_occluder_reset.wgsl".into()
    }
}

#[derive(TypePath)]
struct RTLBlurShader;

impl ComputeShader for RTLBlurShader {
    fn shader() -> ShaderRef {
        "shaders/rtl/rtl_blur.wgsl".into()
    }
}


#[repr(C)]
#[derive(Default, Clone, Copy, ShaderType, Pod, Zeroable)]
pub(crate) struct RTPointLight {
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

#[repr(C)]
#[derive(Default, Clone, Copy, ShaderType, Pod, Zeroable)]
pub(crate) struct Occluder {
    pub pos: Vec2,
    pub shape_id: u32,
    pub is_static: u32,
    pub shape_params: Vec2,
    pub _pad1: Vec2
    
}

impl Occluder {
    pub fn new(pos: Vec2, occluder: LightOccluder, is_static: bool) -> Self {
        let (id, params) = match occluder {
            LightOccluder::Square(size) => (0, Vec2::new(size, size)),
            LightOccluder::Circle(radius) => (1, Vec2::new(radius, radius)),
        };
        let static_flag: u32 = match is_static {
            true => 1,
            false => 0,
        };
        return Self {
            pos,
            shape_id: id,
            shape_params: params,
            is_static: static_flag,
            _pad1: Vec2::default(),
        };
    }
}


#[derive(Resource)]
pub(crate) struct RTLComputeWorker;

impl ComputeWorker for RTLComputeWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {

        //let occluder_map = world.get_resource::<OccluderMap>().expect("could not find occluder map res when building RTLComputeWorker");

        let rays_per_light = 320;
        let ray_workgroup_size = 64;
        let ray_workgroup_count = (rays_per_light + ray_workgroup_size - 1) / ray_workgroup_size;
        

        let worker = AppComputeWorkerBuilder::new(world)
            .add_storage("lighting_output", &[0u32; 1600*1600])
        //.add_texture("occluder_texture", &texture_handle)   // WHAT IF I made the torch handle much darker so multiplying by 4 just makes that normal but flame emissive??
            .add_storage("occluder_mask", &[0u32; 1600*1600])
            .add_storage("occluders", &[Occluder::default(); MAX_OCCLUDERS as usize])
            .add_uniform("occluder_count", &0)
            .add_storage("lights", &[RTPointLight::default(); MAX_LIGHTS as usize])
            .add_uniform("light_count", &0)
            .add_uniform("current_occluder_frame", &0)
            .add_uniform("total_occluder_frames", &OCCLUDER_FRAME_BUDGET)
            .add_uniform("current_light_frame", &0)
            .add_uniform("total_light_frames", &1)
            .add_uniform("buffer_size", &1600)
            .add_storage("intermediate_blur", &[0u32; 1600*1600])
                    .add_uniform("y", &1)
            .add_uniform("noy", &0)
            .add_uniform("is_static_occluder_reset", &0)

            .add_pass::<RTLResetShader>([100, 100, 1], &["lighting_output", "current_light_frame", "total_light_frames", "buffer_size"])
            .add_pass::<RTLOccluderResetShader>([100, 100 / OCCLUDER_FRAME_BUDGET, 1], &["occluder_mask", "current_occluder_frame", "total_occluder_frames", "buffer_size", "is_static_occluder_reset"])
            .add_pass::<RTLOccludeFillShader>([100, 100 / OCCLUDER_FRAME_BUDGET, 1], &["occluder_count", "occluders", "occluder_mask", "current_occluder_frame", "total_occluder_frames"])
            .add_pass::<RTLComputeShader>([ray_workgroup_count, MAX_LIGHTS, 1], &["light_count", "lights", "lighting_output", "occluder_mask"])
            .add_pass::<RTLBlurShader>([100, 100, 1], &["lighting_output", "intermediate_blur", "buffer_size", "noy"])
            .add_pass::<RTLBlurShader>([100, 100, 1], &["intermediate_blur", "lighting_output", "buffer_size", "y"])
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

pub fn update_occluder_current_frame(
    mut occluder_manager: ResMut<OccluderManager>,
    mut worker: ResMut<AppComputeWorker<RTLComputeWorker>>,
) {
    occluder_manager.current_frame = (occluder_manager.current_frame + 1) % OCCLUDER_FRAME_BUDGET;
    worker.write("current_occluder_frame", &occluder_manager.current_frame);
}


#[derive(Resource)]
pub(crate) struct OccluderManager {
    pub occluders: Vec<Occluder>,
    pub current_frame: u32,
    pub static_refresh_frames_remaining: u32
}


pub fn init_occluder_mask(
    mut commands: Commands
) {
    commands.insert_resource(OccluderManager {
        occluders: vec![Occluder::default(); MAX_OCCLUDERS as usize],
        current_frame: 0,
        static_refresh_frames_remaining: OCCLUDER_FRAME_BUDGET,
    });
}

pub(crate) fn refresh_static_occluders(
    query: Query<(), (With<StaticLightOccluder>, Or<(Changed<Transform>, Changed<LightOccluder>)>)>,
    mut occluder_manager: ResMut<OccluderManager>
) {
    if query.iter().count() > 0 {
        occluder_manager.static_refresh_frames_remaining = OCCLUDER_FRAME_BUDGET;
    }
}

/// TODO: Can have an UpdateEvent or I guess just track change diffs myself for this. (will need updates on occluder layout change whether that's mid level from dynamic ones or on new level loaded)
pub(crate) fn write_occluder_buffer(
    query: Query<(&Transform, &LightOccluder, Option<&StaticLightOccluder>)>,
    mut worker: ResMut<AppComputeWorker<RTLComputeWorker>>,
    mut occluder_manager: ResMut<OccluderManager>
) {
    let h = RESOLUTION as f32 / OCCLUDER_FRAME_BUDGET as f32;
    let min_y = occluder_manager.current_frame as f32 * h;
    let max_y = min_y + h;

    let mut count: u32 = 0;
    for (t, occluder, static_opt) in query.iter() {
        if static_opt.is_some() && occluder_manager.static_refresh_frames_remaining == 0 { continue; }
        
        let (occ_min_y, occ_max_y) = match occluder {
            LightOccluder::Square(s) => (t.translation.y - (s / 2.0), t.translation.y + (s / 2.0)),
            LightOccluder::Circle(r) => (t.translation.y - r, t.translation.y + r),
        };
        if occ_min_y <= max_y && occ_max_y >= min_y {
            occluder_manager.occluders[count as usize] = Occluder::new(t.translation.truncate(), *occluder, static_opt.is_some());
            
            count += 1;
            
            if count >= MAX_OCCLUDERS {
                break;
            }
        }

    }

    worker.write_slice("occluders", &occluder_manager.occluders);
    worker.write("occluder_count", &count);
    worker.write("is_static_occluder_reset", &(occluder_manager.static_refresh_frames_remaining.min(1)));
    occluder_manager.static_refresh_frames_remaining = occluder_manager.static_refresh_frames_remaining.saturating_sub(1);
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


// Instead of raising a full update event, could instead set a u32 "full_updates_remaining" = OCCLUDER_FRAME_BUDGET
// This way it will effectively do a full refresh for the next x frames, where x covers the entire lightmap.
// To check if we need to do a full refresh, we simply check if the number is greater than 0

// Will need to reset this to be a full refresh each time we move stage (though not really each stage refresh)

// For quick testing, can just bind a key to trigger a refresh on currently pressed - to see what performance is gained
// will need to tell the compute shader that it is doing a full wipe etc