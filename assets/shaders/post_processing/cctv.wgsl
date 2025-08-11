#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    time: f32,
    chromatic_intensity: f32,
    fisheye_intensity: f32,
    vignette_intensity: f32,
    vignette_start: f32,
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {

    let fisheye_uv = fisheye_distorted_uv(in.uv);

    let chromatic_intensity = settings.chromatic_intensity;
    var c = vec4<f32>(
        textureSample(screen_texture, texture_sampler, fisheye_uv + vec2<f32>(chromatic_intensity, -chromatic_intensity)).r,
        textureSample(screen_texture, texture_sampler, fisheye_uv + vec2<f32>(-chromatic_intensity, 0.0)).g,
        textureSample(screen_texture, texture_sampler, fisheye_uv + vec2<f32>(0.0, chromatic_intensity)).b,
        1.0
    );


    let vignette_intensity = settings.vignette_intensity;
    let vignette_start = settings.vignette_start;

    let uv_sqrd = fisheye_uv * (1.0 - fisheye_uv.yx);
    let vignette = uv_sqrd.x * uv_sqrd.y * vignette_start;
    c *= min(1.0, max(0.0, pow(vignette, vignette_intensity)));


    let stripe_dark_mul = 0.92;
    let stripe_width: f32 = 0.15;
    let speed: f32 = 1.1;
    let shifted_x = -fisheye_uv.y + settings.time * speed;
    let stripe = floor(shifted_x / stripe_width) % 7.0;
    if stripe == 0.0 {
        c *= vec4f(stripe_dark_mul, stripe_dark_mul, stripe_dark_mul, 1.0);
    }


    return c;
}



fn fisheye_distorted_uv(uv: vec2f) -> vec2f {
    let centered_uv = (uv - 0.5) * 2.0;

    let fish_intensity = settings.fisheye_intensity;;
    var fish_uv = vec2f(0.0, 0.0);
    fish_uv.x = (1.0 - centered_uv.y * centered_uv.y) * fish_intensity * centered_uv.x;
    fish_uv.y = (1.0 - centered_uv.x * centered_uv.x) * fish_intensity * centered_uv.y;

    return uv - fish_uv;
}