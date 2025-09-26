#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    something: f32
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@group(0) @binding(3)
var<storage, read_write> lighting_output: array<u32>;

@group(0) @binding(4) var<uniform> view: View;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {

    let new_uv = vec2f(in.uv.x, 1.0 - in.uv.y);

    let clip = vec4f(new_uv * 2.0 - vec2f(1.0), 0.0, 1.0);
    let world_pos = view.world_from_clip * clip;
    let real_world_pos = world_pos.xyz / world_pos.w;

    var c = textureSample(screen_texture, texture_sampler, in.uv);
    let x = u32(real_world_pos.x);
    let y = u32(real_world_pos.y);

    var light_col = get_gaussian_blur_11x11(x, y);
    let ambient = 0.05;

    let ambient_color = vec3<f32>(1.0, 1.0, 1.0);
    let light_rgb = light_col.rgb * light_col.w + ambient_color * ambient;

    let final_rgb = clamp(light_rgb, vec3<f32>(0.0), vec3<f32>(1.0));
    c *= vec4f(final_rgb, 1.0);
    c.a = 1.0;
    return c;


}

fn get_gaussian_blur_11x11(x: u32, y: u32) -> vec4<f32> {
    let width: u32 = 1600u;
    let height: u32 = 1600u;

    var sum_rgb: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    var sum_intensity: f32 = 0.0;
    var weight_sum: f32 = 0.0;

    let weights = array<array<f32, 11>, 11>(
        array<f32, 11>(0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 2.5, 2.0, 1.5, 1.0, 0.5),
        array<f32, 11>(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0),
        array<f32, 11>(1.5, 3.0, 4.5, 6.0, 7.5, 9.0, 7.5, 6.0, 4.5, 3.0, 1.5),
        array<f32, 11>(2.0, 4.0, 6.0, 8.0,10.0,12.0,10.0, 8.0, 6.0, 4.0, 2.0),
        array<f32, 11>(2.5, 5.0, 7.5,10.0,12.5,15.0,12.5,10.0, 7.5, 5.0, 2.5),
        array<f32, 11>(3.0, 6.0, 9.0,12.0,15.0,18.0,15.0,12.0, 9.0, 6.0, 3.0),
        array<f32, 11>(2.5, 5.0, 7.5,10.0,12.5,15.0,12.5,10.0, 7.5, 5.0, 2.5),
        array<f32, 11>(2.0, 4.0, 6.0, 8.0,10.0,12.0,10.0, 8.0, 6.0, 4.0, 2.0),
        array<f32, 11>(1.5, 3.0, 4.5, 6.0, 7.5, 9.0, 7.5, 6.0, 4.5, 3.0, 1.5),
        array<f32, 11>(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0),
        array<f32, 11>(0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 2.5, 2.0, 1.5, 1.0, 0.5)
    );
    for (var dy: i32 = -5; dy <= 5; dy = dy + 1) {
        for (var dx: i32 = -5; dx <= 5; dx = dx + 1) {
            let ix = clamp(i32(x) + dx, 0, i32(width) - 1);
            let iy = clamp(i32(y) + dy, 0, i32(height) - 1);
            let idx = u32(ix) + u32(iy) * width;

            let w = weights[(dy + 5)][(dx + 5)];
            let rgbi = unpack_rgbi(lighting_output[idx]);

            sum_rgb += rgbi.xyz * rgbi.w * w;
            sum_intensity += rgbi.w * w;
            weight_sum += w;
        }
    }

    let final_rgb = sum_rgb / max(sum_intensity, 1.0);
    let final_intensity = sum_intensity / weight_sum;

    return vec4<f32>(final_rgb, final_intensity);
}



fn unpack_rgbi(packed: u32) -> vec4<f32> {
    var r = f32((packed >> 24) & 0x000000FF);
    var g = f32((packed >> 16) & 0x000000FF);
    var b = f32((packed >> 8) & 0x000000FF);
    var intensity = f32(packed & 0x000000FF);
    return vec4f(r, g, b, intensity) / 255.0;
}
