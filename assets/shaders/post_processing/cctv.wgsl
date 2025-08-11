#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    intensity: f32,
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {

    let fisheye_uv = fisheye_distorted_uv(in.uv);

    let offset_strength = settings.intensity;

    return vec4<f32>(
        textureSample(screen_texture, texture_sampler, fisheye_uv + vec2<f32>(offset_strength, -offset_strength)).r,
        textureSample(screen_texture, texture_sampler, fisheye_uv + vec2<f32>(-offset_strength, 0.0)).g,
        textureSample(screen_texture, texture_sampler, fisheye_uv + vec2<f32>(0.0, offset_strength)).b,
        1.0
    );
}



fn fisheye_distorted_uv(uv: vec2f) -> vec2f {
    let centered_uv = (uv - 0.5) * 2.0;

    let fish_intensity = 0.02;
    var fish_uv = vec2f(0.0, 0.0);
    fish_uv.x = (1.0 - centered_uv.y * centered_uv.y) * fish_intensity * centered_uv.x;
    fish_uv.y = (1.0 - centered_uv.x * centered_uv.x) * fish_intensity * centered_uv.y;

    return uv - fish_uv;
}