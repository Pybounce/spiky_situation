
@group(0) @binding(0)
var<uniform> light_count: u32;
@group(0) @binding(1)
var<storage, read> lights: array<RTPointLight>;
@group(0) @binding(2)
var<storage, read> occluder_mask: array<u32>;

@group(0) @binding(3)
var<storage, read_write> red_lightmap: array<atomic<u32>>;
@group(0) @binding(4)
var<storage, read_write> green_lightmap: array<atomic<u32>>;
@group(0) @binding(5)
var<storage, read_write> blue_lightmap: array<atomic<u32>>;


struct RTPointLight {
    pos: vec2<f32>,
    packed_rgb: u32,
    intensity: f32,
};



const PI = 3.14159265359;

@compute @workgroup_size(64, 1)
fn main(@builtin(global_invocation_id) gid : vec3<u32>) {
    let light_idx = gid.y;

    if light_idx >= light_count { return; }

    var cur_pos = lights[light_idx].pos;
    var light_rgbi = vec4f(unpack_rgb(lights[light_idx].packed_rgb).rgb, lights[light_idx].intensity);

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
        let occluder_idx = pos_to_light_idx(cur_pos);

        if occluder_mask[occluder_idx] > 0 {
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
        //let rgb = light_rgbi.rgb * cur_intensity;
        //let current_rgb = lightmap_idx_to_rgb(lightmap_idx);
        
        //let blended_rgb = (rgb + current_rgb) / blended_i;

        //let new_packed = pack_rgbi(vec4f(blended_rgb, blended_i));
        //atomicAdd(&temporal_lightmaps[lightmap_idx], u32(cur_intensity * 100.0));
        //temporal_lightmaps[lightmap_idx] = new_packed;


        atomicAdd(&red_lightmap[lightmap_idx], u32(cur_intensity * light_rgbi.r));
        atomicAdd(&green_lightmap[lightmap_idx], u32(cur_intensity * light_rgbi.g));
        atomicAdd(&blue_lightmap[lightmap_idx], u32(cur_intensity * light_rgbi.b));

        last_pos = vec2<i32>(i32(cur_pos.x), i32(cur_pos.y));

        cur_pos += ray_dir;
        dist += length(ray_dir);
    }


}

fn pos_to_light_idx(pos: vec2f) -> u32 {
    return u32(pos.x) + (1600 * u32(pos.y));
}

fn lightmap_idx_to_rgb(idx: u32) -> vec3f {
    return vec3f(f32(red_lightmap[idx]), f32(green_lightmap[idx]), f32(blue_lightmap[idx]));
}







fn unpack_rgb(packed: u32) -> vec3<f32> {
    var r = f32((packed >> 24) & 0x000000FF);
    var g = f32((packed >> 16) & 0x000000FF);
    var b = f32((packed >> 8) & 0x000000FF);
    return vec3f(r, g, b);
}

fn pack_rgb(rgb: vec3<f32>) -> u32 {
    var packed = u32(0);
    packed |= u32(clamp(rgb.x, 0.0, 255.0)) << 24;
    packed |= u32(clamp(rgb.y, 0.0, 255.0)) << 16;
    packed |= u32(clamp(rgb.z, 0.0, 255.0)) << 8;
    return packed;

}