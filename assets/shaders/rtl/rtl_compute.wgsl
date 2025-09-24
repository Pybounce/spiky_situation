
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

    let ray_count = u32(720);

    let light_idx = gid.x / ray_count;
    let ray_idx = gid.x % ray_count;

    let ray_angle = f32(ray_idx) * (2.0 * PI) / f32(ray_count);
    let ray_dir = vec2f(cos(ray_angle), sin(ray_angle));

    var cur_pos = vec2f(200.0, 80.0);
    var intensity = 1.0;
    var step_falloff = 0.01;

    while intensity > 0.0 {

        let lightmap_idx = u32(cur_pos.x) + (1600 * u32(cur_pos.y));

        if occluder_mask[lightmap_idx] > 0 { return; }
        
        lighting_output[lightmap_idx] = intensity;


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



