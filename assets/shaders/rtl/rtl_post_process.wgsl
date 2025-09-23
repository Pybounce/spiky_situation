#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    something: f32
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@group(0) @binding(3)
var<storage, read_write> lighting_output: array<f32>;

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
    let l = lighting_output[x + (y * 1600)];

    c += (abs(l - 1.0) * 0.1);
    return c;
}
