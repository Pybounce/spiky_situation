@group(0) @binding(0)
var<uniform> light_count: u32;
@group(0) @binding(1)
var<storage, read> lights: array<RTAreaLight>;
@group(0) @binding(2)
var<storage, read> occluder_mask: array<u32>;

@group(0) @binding(3)
var<storage, read_write> red_lightmap: array<atomic<u32>>;
@group(0) @binding(4)
var<storage, read_write> green_lightmap: array<atomic<u32>>;
@group(0) @binding(5)
var<storage, read_write> blue_lightmap: array<atomic<u32>>;

struct RTAreaLight {
    pos: vec2<f32>,
    packed_rgb: u32,
    intensity: f32,
    rect: vec4<f32>, // (min_x, min_y, max_x, max_y)
};

const DEG30: f32 = 0.5235987756 / 2.0; // 30 degrees in radians

@compute @workgroup_size(64, 1)
fn main(@builtin(global_invocation_id) gid : vec3<u32>) {
    var light_rgbi = vec4f(255.0, 255.0, 255.0, 1.0);
    var cur_pos: vec2f;
    var ray_dir: vec2f;

    var ray_accum: u32 = 0u;
    var found = false;

    for (var i = 0u; i < light_count; i++) {
        let rect = lights[i].rect;
        let width = rect.z - rect.x;
        let height = rect.w - rect.y;

        let rays_x = u32(width / 4.0);
        let rays_y = u32(height / 4.0);
        let total_rays = 3u * (rays_x + rays_y) * 2u;

        if rays_x == 0u || rays_y == 0u {
            continue;
        }

        if gid.x < ray_accum + total_rays {
            let local_idx = gid.x - ray_accum;
            let group_idx = local_idx / 3u;
            let sub_ray = local_idx % 3u;

            var angle_offset: f32;
            if sub_ray == 0u {
                angle_offset = 0.0;
            } else if sub_ray == 1u {
                angle_offset = DEG30;
            } else {
                angle_offset = -DEG30;
            }

            if group_idx < rays_x {
                let x = rect.x + f32(group_idx) * 4.0;
                cur_pos = vec2f(x, rect.y);
                ray_dir = vec2f(0.0, -1.0);
            } else if group_idx < rays_x + rays_y {
                let y = rect.y + f32(group_idx - rays_x) * 4.0;
                cur_pos = vec2f(rect.z - 8.0, y);
                ray_dir = vec2f(1.0, 0.0);
            } else if group_idx < rays_x * 2u + rays_y {
                let x = rect.z - f32(group_idx - rays_x - rays_y) * 4.0;
                cur_pos = vec2f(x, rect.w);
                ray_dir = vec2f(0.0, 1.0);
            } else {
                let y = rect.w - f32(group_idx - rays_x * 2u - rays_y) * 4.0;
                cur_pos = vec2f(rect.x + 8.0, y);
                ray_dir = vec2f(-1.0, 0.0);
            }

            ray_dir = rotate(ray_dir, angle_offset);
            cur_pos += lights[i].pos - vec2f(width / 2.0, height / 2.0);
            light_rgbi = vec4f(unpack_rgb(lights[i].packed_rgb).rgb, lights[i].intensity);
            if sub_ray == 0u {
                light_rgbi.w *= 1.5;
            }
            found = true;
            break;
        }

        ray_accum += total_rays;
    }

    if !found {
        return;
    }

    var last_pos = vec2<i32>(0, 0);
    var last_was_occ = false;
    var dist = 0.0;

    while dist < 500.0 {
        let lightmap_idx = pos_to_light_idx(cur_pos);
        let occluder_idx = pos_to_light_idx(cur_pos);

        if occluder_mask[occluder_idx] > 0u {
            if last_was_occ {
                return;
            }
            last_was_occ = true;
            light_rgbi.w *= 0.7;

            if abs(i32(cur_pos.x) - last_pos.x) == 1 {
                ray_dir.x = -ray_dir.x;
            } else if abs(i32(cur_pos.y) - last_pos.y) == 1 {
                ray_dir.y = -ray_dir.y;
            } else {
                return;
            }

        } else {
            last_was_occ = false;
        }

        let falloff = exp(-dist * 0.04);
        let cur_intensity = light_rgbi.w * falloff;
        if cur_intensity <= 0.01 {
            break;
        }

        atomicAdd(&red_lightmap[lightmap_idx], u32(cur_intensity * light_rgbi.r));
        atomicAdd(&green_lightmap[lightmap_idx], u32(cur_intensity * light_rgbi.g));
        atomicAdd(&blue_lightmap[lightmap_idx], u32(cur_intensity * light_rgbi.b));

        last_pos = vec2<i32>(i32(cur_pos.x), i32(cur_pos.y));
        cur_pos += ray_dir;
        dist += length(ray_dir);
    }
}

fn pos_to_light_idx(pos: vec2f) -> u32 {
    return u32(pos.x) + (1600u * u32(pos.y));
}

fn unpack_rgb(packed: u32) -> vec3<f32> {
    let r = f32((packed >> 24) & 0xFF);
    let g = f32((packed >> 16) & 0xFF);
    let b = f32((packed >> 8) & 0xFF);
    return vec3f(r, g, b);
}

fn rotate(v: vec2f, angle: f32) -> vec2f {
    let cos_a = cos(angle);
    let sin_a = sin(angle);
    return vec2f(
        v.x * cos_a - v.y * sin_a,
        v.x * sin_a + v.y * cos_a
    );
}
