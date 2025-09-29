
@group(0) @binding(0)
var<storage, read> source: array<u32>;
@group(0) @binding(1)
var<storage, read_write> dest: array<u32>;
@group(0) @binding(2)
var<uniform> buffer_size: u32;
@group(0) @binding(3)
var<uniform> is_y: u32;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) gid : vec3<u32>) {

    let idx = gid.x + gid.y * buffer_size;
    let w2 = array<f32, 11>(
        0.009300040045324049, 
        0.028001560233780885, 
        0.06598396774984912, 
        0.12170274650962626, 
        0.17571363439579307, 
        0.19859610213125314, 
        0.17571363439579307, 
        0.12170274650962626, 
        0.06598396774984912, 
        0.028001560233780885, 
        0.009300040045324049
    );
    let w = array<f32, 11>(0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 2.5, 2.0, 1.5, 1.0, 0.5);

    var sum_rgb = vec3f(0.0);
    var sum_i = 0.0;
    var sum_w = 0.0;
    let r = 5;

    for (var i: i32 = -r; i <= r; i++) {
        var sample_idx = 0;
        let sample_x = i32(gid.x) + i;
        sample_idx = sample_x + i32(gid.y) * i32(buffer_size);

        if is_y == 1 {
            let sample_y = i32(gid.y) + i;
            sample_idx = i32(gid.x) + sample_y * i32(buffer_size);
        }


        if sample_idx >= 0 && sample_idx < i32(buffer_size * buffer_size) {
            let sample_rgbi = unpack_rgbi(source[u32(sample_idx)]);
            let w = w[u32(i + r)];
            sum_rgb += sample_rgbi.rgb * sample_rgbi.w * w;
            sum_i += sample_rgbi.w * w;
            sum_w += w;
        }
    }

    //let dst_xy = vec2(buffer_size - 1 - gid.y, gid.x);
    //let dst_idx = dst_xy.x + dst_xy.y * buffer_size;
    //dest[idx] = pack_rgbi(vec4f(sum_rgb, sum_i)); 



    dest[idx] = pack_rgbi(vec4f(sum_rgb / max(1.0, sum_i), sum_i / sum_w)); 
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
