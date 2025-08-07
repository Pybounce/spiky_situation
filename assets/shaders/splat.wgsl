

#import bevy_sprite::mesh2d_view_bindings::globals

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> current_time: f32;


@group(2) @binding(1)
var blood_texture: texture_2d<f32>;

@group(2) @binding(2)
var blood_sampler: sampler;


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {

    let reveal_duration = 0.1;
    let time_passed = globals.time - current_time;
    let center = vec2<f32>(0.5, 0.5);
    let dist = distance(mesh.uv, center);
    let reveal_time = dist / 0.707 * reveal_duration;
    if (time_passed < reveal_time) {
        discard;
    }

    let shifted_uv = mesh.uv;
    let tex_color = textureSample(blood_texture, blood_sampler, shifted_uv);


    return tex_color;
}
