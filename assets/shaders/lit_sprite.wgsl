
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var albedo_texture: texture_2d<f32>;
@group(2) @binding(1)
var albedo_sampler: sampler;
@group(2) @binding(2)
var specular_texture: texture_2d<f32>;
@group(2) @binding(3)
var specular_sampler: sampler;
@group(2) @binding(4) var<uniform> uv_rect: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let tex_size = vec2<f32>(textureDimensions(albedo_texture));
    let normalised_rect = uv_rect.xyzw / tex_size.xyxy;
    let uv = vec2f(mix(normalised_rect.x, normalised_rect.z, mesh.uv.x), mix(normalised_rect.y, normalised_rect.w, mesh.uv.y));
    var tex_color = textureSample(albedo_texture, albedo_sampler, uv);
    return tex_color;

}