
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

        let lightmap_idx = pos_to_light_idx(cur_pos);

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
        
        let falloff = exp(-dist * 0.015);
        lighting_output[lightmap_idx] += u32(intensity * 100.0 * falloff);
        
        //lighting_output[pos_to_light_idx(cur_pos + vec2f(1.0, 0.0))] += u32(intensity * 100.0 * falloff);
        //lighting_output[pos_to_light_idx(cur_pos - vec2f(1.0, 0.0))] += u32(intensity * 100.0 * falloff);
        //lighting_output[pos_to_light_idx(cur_pos + vec2f(0.0, 1.0))] += u32(intensity * 100.0 * falloff);
        //lighting_output[pos_to_light_idx(cur_pos - vec2f(0.0, 1.0))] += u32(intensity * 100.0 * falloff);
    //
        //lighting_output[pos_to_light_idx(cur_pos + vec2f(1.0, 1.0))] += u32(intensity * 100.0 * falloff);
        //lighting_output[pos_to_light_idx(cur_pos - vec2f(1.0, 1.0))] += u32(intensity * 100.0 * falloff);
        //lighting_output[pos_to_light_idx(cur_pos + vec2f(-1.0, 1.0))] += u32(intensity * 100.0 * falloff);
        //lighting_output[pos_to_light_idx(cur_pos - vec2f(-1.0, 1.0))] += u32(intensity * 100.0 * falloff);


        last_pos = vec2<i32>(i32(cur_pos.x), i32(cur_pos.y));

        cur_pos += ray_dir;
        dist += length(ray_dir);
    }


}

fn pos_to_light_idx(pos: vec2f) -> u32 {
    return u32(pos.x) + (1600 * u32(pos.y));
}



// OK NEXT UP
//
// Multiple Blur Passes! :D
//      Look up how guassian blur really works
//      Look up working groups memory
//      See if adding multiple blur passes helps smooth out the lighting
//          It might end up that a large kernal blur (ie 11x11) is better than multiple 5x5 blurs but will need to find out
//
// Implement ray spawn bounces
//      Probably have a ray buffer that contains queued rays
//      Not sure if I should be, in the initial pass, filling it and then have 2 passes going through it, could work.
//      So that initial one would have the amount of invocations as there are lights? Or maybe I keep it exactly how it is and each thread just writes a ray to the buffer
//      Then we have a new raymarch pass where rays are taken from the buffer and processed
//          When one of these rays in the first pass hits an occluder, it adds another N rays to the buffer in the allocated slots
//          Will need to create the buffer and index mapper based on (MAX_LIGHTS * RAYS_PER_LIGHT) + (MAX_LIGHTS * RAYS_PER_LIGHT * RAYS_PER_BOUNCE) ^ BOUNCE_COUNT
//              Since this is a large buffer, the struct should be optimised, very.
//          When a ray is done, it should mark itself as finished in the buffer.
