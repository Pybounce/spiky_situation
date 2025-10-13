
@group(0) @binding(0)
var<storage, read> temporal_buffers: array<u32>;
@group(0) @binding(1)
var<uniform> temporal_buffer_count: u32;
@group(0) @binding(2)
var<uniform> buffer_size: u32;
@group(0) @binding(3)
var<storage, read_write> output_buffer: array<u32>;
@group(0) @binding(4)
var<uniform> temporal_lightmap_index: u32;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) gid : vec3<u32>) {

    let base_idx = gid.x + gid.y * buffer_size;
    var rgbi_sum = vec4f(0.0, 0.0, 0.0, 0.0);
    var current_rgbi = unpack_rgbi(temporal_buffers[base_idx + (buffer_size * buffer_size * temporal_lightmap_index)]);

    for (var i: u32 = 0u; i < temporal_buffer_count; i = i + 1u) {
        let idx = base_idx + (buffer_size * buffer_size * i);
        rgbi_sum += clamp(unpack_rgbi(temporal_buffers[idx]), current_rgbi - 100.0, current_rgbi + 100.0);
        //rgbi_sum += unpack_rgbi(temporal_buffers[idx]);
    }

    rgbi_sum = rgbi_sum / f32(temporal_buffer_count);

    output_buffer[base_idx] = pack_rgbi(rgbi_sum);



}


fn unpack_rgbi(packed: u32) -> vec4<f32> {
    var r = f32((packed >> 24) & 0x000000FF);
    var g = f32((packed >> 16) & 0x000000FF);
    var b = f32((packed >> 8) & 0x000000FF);
    var intensity = f32(packed & 0x000000FF);
    return vec4f(r, g, b, intensity);
}

fn pack_rgbi(rgbi: vec4<f32>) -> u32 {
    var packed = u32(0);
    packed |= u32(clamp(rgbi.x, 0.0, 255.0)) << 24;
    packed |= u32(clamp(rgbi.y, 0.0, 255.0)) << 16;
    packed |= u32(clamp(rgbi.z, 0.0, 255.0)) << 8;
    packed |= u32(clamp(rgbi.w, 0.0, 255.0));
    return packed;
}