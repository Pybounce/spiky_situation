
@group(0) @binding(0)
var<uniform> occluder_count: u32;
@group(0) @binding(1)
var<storage, read> occluders: array<Occluder>;
@group(0) @binding(2)
var<storage, read_write> occluder_mask: array<u32>;


struct Occluder {
    pos: vec2<f32>,
    shape_id: u32,
    _pad0: u32,
    shape_params: vec2<f32>,
    _pad1: vec2<f32>,
};



@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) gid : vec3<u32>) {
    if occluder_count == 0u { return; }

    for (var occluder_i: u32 = 0u; occluder_i < occluder_count; occluder_i++) {
        let occluder = occluders[occluder_i];

        if occluder.shape_id == 0u {
            let dist = dist_to_square(vec2f(f32(gid.x), f32(gid.y)), occluder.shape_params / 2.0, occluder.pos);
            if dist <= 0.0 {
                occluder_mask[gid.x + (gid.y * 1600)] = 1u;
            }
        }
        else if occluder.shape_id == 1u {
            let dist = dist_to_circle(vec2f(f32(gid.x), f32(gid.y)), occluder.shape_params.x, occluder.pos);
            if dist <= 0.0 {
                occluder_mask[gid.x + (gid.y * 1600)] = 1u;
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

