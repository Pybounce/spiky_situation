
@group(0) @binding(0)
var<uniform> uni: f32;
@group(0) @binding(1)
var<storage, read_write> lighting_output: array<f32>;
@group(0) @binding(2)
var<storage, read> occluder_mask: array<u32>;


const PI = 3.141;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid : vec3<u32>) {
    let _u = uni;
    let _o = occluder_mask[0];
    let _l = lighting_output[0];

    let ray_count = 360;

    let light_idx = gid.x / 360;
    let ray_idx = gid.x % 360;

    let ray_angle = f32(ray_idx) * (2.0 * PI) / f32(ray_count);
    let ray_dir = vec2f(cos(ray_angle), sin(ray_angle));

    var cur_pos = vec2f(200.0, 80.0);
    var intensity = 0.1;
    var step_falloff = 0.001;

    var ray_end = false;

    while intensity > 0.0 {

        let lightmap_idx = u32(cur_pos.x) + (1600 * u32(cur_pos.y));

        if occluder_mask[lightmap_idx] > 0 { ray_end = true; }
        
        lighting_output[lightmap_idx] += intensity;

        if ray_end {
            lighting_output[lightmap_idx] = 0.0;
        }

        cur_pos += ray_dir;
        intensity -= step_falloff;
    }



    //let lightmap_idx = gid.x + (1600 * gid.y);
    //let light_pos = vec2f(200.0, 80.0);
    //let pos = vec2f(f32(gid.x), f32(gid.y));
    //let dist = length(light_pos - pos);
    //let intensity = smoothstep(64.0, 0.0, dist);
    //lighting_output[lightmap_idx] = intensity;
}



