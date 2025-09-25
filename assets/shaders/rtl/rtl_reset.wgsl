
@group(0) @binding(0)
var<storage, read_write> lighting_output: array<u32>;


@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) gid : vec3<u32>) {
    let lightmap_idx = gid.x + (1600 * gid.y);
    lighting_output[lightmap_idx] = 0;
}



