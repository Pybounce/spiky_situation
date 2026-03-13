
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var albedo_texture: texture_2d<f32>;
@group(2) @binding(1)
var albedo_sampler: sampler;
@group(2) @binding(2)
var specular_texture: texture_2d<f32>;
@group(2) @binding(3)
var specular_sampler: sampler;


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {

    var tex_color = textureSample(albedo_texture, albedo_sampler, mesh.uv);
    return tex_color;

}