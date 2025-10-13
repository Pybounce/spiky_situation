
@group(0) @binding(0)
var<storage, read_write> buffer: array<u32>;
//@group(0) @binding(1)
//var<uniform> temporal_lightmap_index: u32;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) gid : vec3<u32>) {

    let idx = gid.x + (1600 * gid.y);// + (temporal_lightmap_index * 1600 * 1600);

    buffer[idx] = 0;
}

