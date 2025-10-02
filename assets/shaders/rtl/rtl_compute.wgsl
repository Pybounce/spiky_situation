
@group(0) @binding(0)
var<uniform> light_count: u32;
@group(0) @binding(1)
var<storage, read> lights: array<RTPointLight>;
@group(0) @binding(2)
var<storage, read_write> lighting_output: array<u32>;
@group(0) @binding(3)
var<storage, read> occluder_mask: array<u32>;


struct RTPointLight {
    pos: vec2<f32>,
    packed_light: u32,
    _pad: u32,
};



const PI = 3.14159265359;

@compute @workgroup_size(64, 1)
fn main(@builtin(global_invocation_id) gid : vec3<u32>) {
    let light_idx = gid.y;

    if light_idx >= light_count { return; }

    var cur_pos = lights[light_idx].pos;
    var light_rgbi = unpack_rgbi(lights[light_idx].packed_light);

    let ray_count = u32(320);

    let ray_idx = gid.x % ray_count;

    var ray_angle = f32(ray_idx) * (2.0 * PI) / f32(ray_count);
    var ray_dir = vec2f(cos(ray_angle), sin(ray_angle));

    
    var step_falloff = 0.997;

    var last_pos = vec2<i32>(0, 0);
    var last_was_occ = false;

    var dist = 0.0;
    while dist < 500.0 {

        let lightmap_idx = pos_to_light_idx(cur_pos);

        if occluder_mask[lightmap_idx] > 0 {
            if last_was_occ { return; }
            else { last_was_occ = true; }
            light_rgbi.w *= 0.7;
            if abs(i32(cur_pos.x) - last_pos.x) == 1 {
                ray_dir.x = -ray_dir.x;
            }
            else if abs(i32(cur_pos.y) - last_pos.y) == 1 {
                ray_dir.y = -ray_dir.y;
            }
            else {
                return;
            }
        }
        else { last_was_occ = false; }
        
        let falloff = exp(-dist * 0.01);
        let cur_intensity = light_rgbi.w * falloff;
        if cur_intensity <= 0.01 { break; }
        let rgb = light_rgbi.rgb * cur_intensity;
        let current_rgbi = unpack_rgbi(lighting_output[lightmap_idx]);
        let current_rgb = current_rgbi.rgb * current_rgbi.w;
        
        let blended_i = current_rgbi.w + cur_intensity;
        let blended_rgb = (rgb + current_rgb) / blended_i;

        let new_packed = pack_rgbi(vec4f(blended_rgb, blended_i));
        //atomicAdd(&lighting_output[lightmap_idx], u32(cur_intensity * 100.0));
        lighting_output[lightmap_idx] = new_packed;

        last_pos = vec2<i32>(i32(cur_pos.x), i32(cur_pos.y));

        cur_pos += ray_dir;
        dist += length(ray_dir);
    }


}

fn pos_to_light_idx(pos: vec2f) -> u32 {
    return u32(pos.x) + (1600 * u32(pos.y));
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