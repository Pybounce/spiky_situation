
@group(0) @binding(0)
var<uniform> uni: f32;
@group(0) @binding(1)
var<storage, read_write> lighting_output: array<u32>;
@group(0) @binding(2)
var<storage, read> occluder_mask: array<u32>;


const PI = 3.14159265359;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid : vec3<u32>) {
    let _u = uni;
    let _o = occluder_mask[0];
    let _l = lighting_output[0];

    let ray_count = u32(360);

    let light_idx = gid.x / ray_count;
    let ray_idx = gid.x % ray_count;

    var ray_angle = f32(ray_idx) * (2.0 * PI) / f32(ray_count);
    var ray_dir = vec2f(cos(ray_angle), sin(ray_angle));

    var cur_pos = vec2f(200.0, 120.0);
    var intensity = 0.9;
    var step_falloff = 0.997;

    var last_pos = vec2<i32>(0, 0);
    var last_was_occ = false;

    var dist = 0.0;
    while dist < 500.0 {

        let lightmap_idx = u32(cur_pos.x) + (1600 * u32(cur_pos.y));

        if occluder_mask[lightmap_idx] > 0 {
            if last_was_occ { return; }
            else { last_was_occ = true; }

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
        
        //let falloff = 1.0 / (1.0 + dist * dist * 0.1);
        //let falloff = 1.0 / (1.0 + dist * dist * 0.001);
        let falloff = exp(-dist * 0.015);
        lighting_output[lightmap_idx] += u32(intensity * 100.0 * falloff);



        last_pos = vec2<i32>(i32(cur_pos.x), i32(cur_pos.y));

        cur_pos += ray_dir;
        //intensity *= step_falloff;
        //intensity -= 0.001;
        dist += length(ray_dir);
    }



    //let lightmap_idx = gid.x + (1600 * gid.y);
    //let light_pos = vec2f(200.0, 80.0);
    //let pos = vec2f(f32(gid.x), f32(gid.y));
    //let dist = length(light_pos - pos);
    //let intensity = smoothstep(64.0, 0.0, dist);
    //lighting_output[lightmap_idx] = intensity;
}

fn spawn_ray(in_intensity: f32, in_cur_pos: vec2<i32>, in_step_falloff: f32, in_ray_dir: vec2f, in_dist: f32) {
    var ray_dir = in_ray_dir;
    var cur_pos = vec2f(f32(in_cur_pos.x), f32(in_cur_pos.y));
    var intensity = in_intensity;
    var step_falloff = in_step_falloff;

    var last_pos = vec2<i32>(0, 0);
    var last_was_occ = false;
    var dist = 0.0;
    while dist < 300.0 {

        let lightmap_idx = u32(cur_pos.x) + (1600 * u32(cur_pos.y));

        if occluder_mask[lightmap_idx] > 0 {
            if last_was_occ { return; }
            else { last_was_occ = true; }
            if abs(i32(cur_pos.x) - last_pos.x) == 1 {
                ray_dir.x = -ray_dir.x;
            } else if abs(i32(cur_pos.y) - last_pos.y) == 1 {
                ray_dir.y = -ray_dir.y;
            }
            else {
                return;
            }
        }
        else { last_was_occ = false; }
        
        let falloff = exp(-dist * 0.02);
        for (var dx = -1; dx <= 1; dx++) {
            for (var dy = -1; dy <= 1; dy++) {
                let idx = (i32(cur_pos.x) + dx) + (i32(cur_pos.y) + dy) * 1600;
                lighting_output[idx] += u32(intensity * 100.0 * falloff);

            }
        }
        last_pos = vec2<i32>(i32(cur_pos.x), i32(cur_pos.y));

        cur_pos += ray_dir;
        //intensity *= step_falloff;
        //intensity -= 0.001;
        dist += length(ray_dir);
    }
}