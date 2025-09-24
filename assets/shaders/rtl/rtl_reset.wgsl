
@group(0) @binding(0)
var<uniform> uni: f32;
@group(0) @binding(1)
var<storage, read_write> lighting_output: array<f32>;


@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) gid : vec3<u32>) {
    let _u = uni;
    let _l = lighting_output[0];

    let lightmap_idx = gid.x + (1600 * gid.y);
    lighting_output[lightmap_idx] = 0.0;
}



