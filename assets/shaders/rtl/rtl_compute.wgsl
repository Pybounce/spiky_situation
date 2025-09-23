
@group(0) @binding(0)
var<uniform> uni: f32;
@group(0) @binding(1)
var<storage, read_write> lighting_output: array<f32>;
@group(0) @binding(2)
var<storage, read> occluder_mask: array<u32>;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let _u = uni;
    lighting_output[invocation_id.x + (1600 * invocation_id.y)] = f32(occluder_mask[invocation_id.x + (1600 * invocation_id.y)]);
}



