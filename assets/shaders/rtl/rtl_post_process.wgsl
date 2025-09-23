#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    something: f32
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@group(0) @binding(3)
var<storage, read_write> lighting_output: array<f32>;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {

    var c = textureSample(screen_texture, texture_sampler, in.uv);
    if in.uv.x <= 0.25 {
        let l = lighting_output[u32(1600.0 * in.uv.x * 4.0) + u32(1600.0 * 1600.0 * in.uv.y)];
        c = c * l;
    }
    return c;
}
