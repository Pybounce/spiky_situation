
@group(0) @binding(0)
var<uniform> uni: f32;
@group(0) @binding(1)
var<storage, read_write> lighting_output: array<f32>;
@group(0) @binding(2)
var<storage, read> occluder_mask: array<u32>;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) gid : vec3<u32>) {
    let _u = uni;
    let _o = occluder_mask[0];

    let lightmap_idx = gid.x + (1600 * gid.y);


    let light_pos = vec2f(200.0, 80.0);
    let pos = vec2f(f32(gid.x), f32(gid.y));

    let dist = length(light_pos - pos);

    let intensity = smoothstep(64.0, 0.0, dist);


    lighting_output[lightmap_idx] = intensity;
}



