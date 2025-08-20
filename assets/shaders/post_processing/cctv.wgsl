#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    time: f32,
    chromatic_intensity: f32,
    fisheye_intensity: f32,
    vignette_intensity: f32,
    vignette_start: f32,
    scanline_dark_mul: f32,
    scanline_width: f32,
    scanline_speed: f32,
    scanline_gap: f32,

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
    let vignette_output = min(1.0, max(0.0, pow(vignette, vignette_intensity)));



    let stripe_dark_mul = settings.scanline_dark_mul; //0.95;
    let stripe_width: f32 = settings.scanline_width;// 0.15;
    let speed: f32 = settings.scanline_speed; //0.7;
    let gap: f32 = 7.0;// settings.scanline_gap; currently broken when passing in 7.0 do not know why

    let shifted_x = -fisheye_uv.y + settings.time * speed;
    let stripe = floor(shifted_x / stripe_width) % gap;
    var scanline_output = 1.0;
    if stripe == 0.0 {
        scanline_output = stripe_dark_mul;
    }

    let vertical_line_mul = vertical_lines(fisheye_uv.x);
    c *= vertical_line_mul;

    c *= min(vignette_output, scanline_output);
    c.a = 1.0;
    return c;
}

fn vertical_lines(x_uv: f32) -> f32 {
    let stripe_dark_mul = 0.96;
    let stripe_width: f32 = 0.002;
    let gap: f32 = 2.0;

    let stripe = floor(x_uv / stripe_width) % gap;
    var scanline_output = 1.0;
    if stripe == 0.0 {
        scanline_output = stripe_dark_mul;
    }
    return scanline_output;
}


fn fisheye_distorted_uv(uv: vec2f) -> vec2f {
    let centered_uv = (uv - 0.5) * 2.0;

    let fish_intensity = settings.fisheye_intensity;
    var fish_uv = vec2f(0.0, 0.0);
    fish_uv.x = (1.0 - centered_uv.y * centered_uv.y) * fish_intensity * centered_uv.x;
    fish_uv.y = (1.0 - centered_uv.x * centered_uv.x) * fish_intensity * centered_uv.y;

    return uv - fish_uv;
}