

#import bevy_sprite::mesh2d_view_bindings::globals


struct FragmentInput {
    @location(0) world_pos: vec2<f32>, // World-space position passed from vertex shader
};

struct FragmentOutput {
    @location(0) color: vec4<f32>,
};


@fragment
fn fragment(in: FragmentInput) -> FragmentOutput {
    let stripe_width: f32 = 100.0;

    let shifted_x = in.world_pos.x + in.world_pos.y + globals.time * 20.0;

    let stripe = floor(shifted_x / stripe_width) % 2.0;

    var color: vec4<f32>;
    if stripe == 0.0 {
        color = vec4<f32>(0.8, 0.8, 0.8, 1.0);
    } else {
        color = vec4<f32>(0.8, 0.8, 0.8, 1.0);
    }


    return FragmentOutput(color);
}
