

#import bevy_sprite::mesh2d_view_bindings::globals

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> current_time: f32;


@group(2) @binding(1)
var blood_texture: texture_2d<f32>;

@group(2) @binding(2)
var blood_sampler: sampler;

@group(2) @binding(3)
var<uniform> atlas_rect: vec4f;


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {

    let time_passed = globals.time - current_time;
    let shifted_uv = mix(atlas_rect.xy, atlas_rect.zw, mesh.uv);
    var tex_color = textureSample(blood_texture, blood_sampler, shifted_uv);

    // ok this is stupid but basically for each number of r, that should give it an 10ms delay
    if time_passed * 1000.0 < tex_color.r * 255.0 * 10.0 {
        tex_color.a = 0.0;
    }
    tex_color = vec4f(0.3, 0.0, 0.0, tex_color.a);
    return tex_color;

}
