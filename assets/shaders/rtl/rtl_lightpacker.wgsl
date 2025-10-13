
@group(0) @binding(0)
var<storage, read_write> buffer: array<u32>;
@group(0) @binding(1)
var<storage, read_write> red_lightmap: array<u32>;
@group(0) @binding(2)
var<storage, read_write> green_lightmap: array<u32>;
@group(0) @binding(3)
var<storage, read_write> blue_lightmap: array<u32>;



@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) gid : vec3<u32>) {

    let idx = gid.x + (1600 * gid.y);
    let rgb = vec3f(f32(red_lightmap[idx]), f32(green_lightmap[idx]), f32(blue_lightmap[idx]));
    let i = length(rgb);
    let rgbi = vec4f(normalize(rgb), i / 150000.0);
    buffer[idx] = pack_rgbi(rgbi);
}


fn unpack_rgbi(packed: u32) -> vec4<f32> {
    var r = f32((packed >> 24) & 0x000000FF);
    var g = f32((packed >> 16) & 0x000000FF);
    var b = f32((packed >> 8) & 0x000000FF);
    var intensity = f32(packed & 0x000000FF);
    return vec4f(r, g, b, intensity) / 255.0;
}

fn pack_rgbi(rgbi: vec4<f32>) -> u32 {
    var packed = u32(0);
    packed |= u32(clamp(rgbi.x * 255.0, 0.0, 255.0)) << 24;
    packed |= u32(clamp(rgbi.y * 255.0, 0.0, 255.0)) << 16;
    packed |= u32(clamp(rgbi.z * 255.0, 0.0, 255.0)) << 8;
    packed |= u32(clamp(rgbi.w * 255.0, 0.0, 255.0));
    return packed;

}