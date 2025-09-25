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
    let x = u32(real_world_pos.x) + 8;
    let y = u32(real_world_pos.y) + 8;

    let l = get_gaussian_blur_3x3(x, y) / 100.0;
    let ambient = 0.05;

    c *= vec4f(1.0, 230.0/255.0, 205.0/255.0, 1.0) * min(1.0, l + ambient);
    c.a = 1.0;
    return c;
}


fn get_gaussian_blur_3x3(x: u32, y: u32) -> f32 {
    let width: u32 = 1600u;

    let x0 = max(x - 1u, 0u);
    let x1 = x;
    let x2 = min(x + 1u, width - 1u);

    let y0 = max(y - 1u, 0u);
    let y1 = y;
    let y2 = min(y + 1u, width - 1u);

    let p00 = f32(lighting_output[x0 + y0 * width]);
    let p01 = f32(lighting_output[x1 + y0 * width]);
    let p02 = f32(lighting_output[x2 + y0 * width]);

    let p10 = f32(lighting_output[x0 + y1 * width]);
    let p11 = f32(lighting_output[x1 + y1 * width]);
    let p12 = f32(lighting_output[x2 + y1 * width]);

    let p20 = f32(lighting_output[x0 + y2 * width]);
    let p21 = f32(lighting_output[x1 + y2 * width]);
    let p22 = f32(lighting_output[x2 + y2 * width]);

    let sum = p00 * 1.0 + p01 * 2.0 + p02 * 1.0 +
              p10 * 2.0 + p11 * 4.0 + p12 * 2.0 +
              p20 * 1.0 + p21 * 2.0 + p22 * 1.0;

    return sum / 16.0;
}
