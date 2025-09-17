

#import bevy_sprite::mesh2d_view_bindings::globals


struct FragmentInput {
    @location(0) world_pos: vec2<f32>, // World-space position passed from vertex shader
};

struct FragmentOutput {
    @location(0) color: vec4<f32>,
};


@fragment
fn fragment(in: FragmentInput) -> FragmentOutput {
    let stripe_width: f32 = 16.0 * 4.0;

    let shifted_x = in.world_pos.x + in.world_pos.y;// + globals.time * 0.0;

    let stripe = floor(shifted_x / stripe_width) % 2.0;

    let is_coloured = (floor(in.world_pos.x / stripe_width) + floor(in.world_pos.y / stripe_width)) % 2.0 == 0.0;

    let colour_mul = 0.9;
    var color: vec4<f32>;
    if is_coloured {
        color = vec4<f32>(0.4, 0.35, 0.22, 1.0);
    } else {
        color = vec4<f32>(0.4, 0.35, 0.22, 1.0);
    }


    return FragmentOutput(color);
}
