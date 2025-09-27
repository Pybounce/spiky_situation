
@group(0) @binding(0)
var<uniform> occluder_count: u32;
@group(0) @binding(1)
var<storage, read> occluders: array<Occluder>;
@group(0) @binding(2)
var<storage, read_write> occluder_mask: array<u32>;
@group(0) @binding(3)
var<uniform> current_frame: u32;
@group(0) @binding(4)
var<uniform> total_frames: u32;


struct Occluder {
    pos: vec2<f32>,
    shape_id: u32,
    _pad0: u32,
    shape_params: vec2<f32>,
    _pad1: vec2<f32>,
};



@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) old_gid : vec3<u32>) {
    if occluder_count == 0u { return; }
    let gid = get_buffer_coords(old_gid.xy, current_frame, total_frames, 1600u);

    for (var occluder_i: u32 = 0u; occluder_i < occluder_count; occluder_i++) {
        let occluder = occluders[occluder_i];

        if occluder.shape_id == 0u {
            let dist = dist_to_square(vec2f(f32(gid.x), f32(gid.y)), occluder.shape_params / 2.0, occluder.pos);
            if dist <= 0.0 {
                occluder_mask[gid.x + (gid.y * 1600)] = 1u;
                return;
            }
        }
        else if occluder.shape_id == 1u {
            let dist = dist_to_circle(vec2f(f32(gid.x), f32(gid.y)), occluder.shape_params.x, occluder.pos);
            if dist <= 0.0 {
                occluder_mask[gid.x + (gid.y * 1600)] = 1u;
                return;
            }
        }

    }
}



fn dist_to_square(eval_pos: vec2f, dimentions: vec2f, box_pos: vec2f) -> f32
{
    let d = abs(eval_pos - box_pos) - dimentions;
    return length(max(d,vec2f(0.0, 0.0))) + min(max(d.x,d.y),0.0);
}

fn dist_to_circle(eval_pos: vec2f, radius: f32, circle_pos: vec2f) -> f32 {
    return length(eval_pos - circle_pos) - radius;
}

fn get_buffer_coords(
    gid: vec2<u32>,
    current_frame: u32,
    total_frames: u32,
    buffer_size: u32
) -> vec2<u32> {
    let slice_height = buffer_size / total_frames;
    let start_y = current_frame * slice_height;
    let global_y = start_y + gid.y;


    return vec2<u32>(gid.x, global_y);
}