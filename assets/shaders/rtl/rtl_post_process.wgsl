#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    something: f32
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@group(0) @binding(3)
var<storage, read_write> lighting_output: array<u32>;

@group(0) @binding(4) var<uniform> view: View;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {

    let new_uv = vec2f(in.uv.x, 1.0 - in.uv.y);

    let clip = vec4f(new_uv * 2.0 - vec2f(1.0), 0.0, 1.0);
    let world_pos = view.world_from_clip * clip;
    let real_world_pos = world_pos.xyz / world_pos.w;

    var c = textureSample(screen_texture, texture_sampler, in.uv);
    let base_x = u32(real_world_pos.x);
    let base_y = u32(real_world_pos.y);

    let dx = real_world_pos.x - f32(base_x);
    let dy = real_world_pos.y - f32(base_y);

    let i00 = unpack_rgbi(lighting_output[base_x + (base_y * 1600u)]);
    let i10 = unpack_rgbi(lighting_output[base_x + 1u + (base_y * 1600u)]);
    let i01 = unpack_rgbi(lighting_output[base_x + ((base_y + 1u) * 1600u)]);
    let i11 = unpack_rgbi(lighting_output[(base_x + 1u) + ((base_y + 1u) * 1600u)]);

    let rgb00 = i00.rgb * i00.w;
    let rgb10 = i10.rgb * i10.w;
    let rgb01 = i01.rgb * i01.w;
    let rgb11 = i11.rgb * i11.w;
    
    let ix0 = mix(rgb00, rgb10, dx);
    let ix1 = mix(rgb01, rgb11, dx);
    let light_rgb = mix(ix0, ix1, dy);

    let ambient = 0.05;
    let ambient_color = vec3<f32>(1.0, 1.0, 1.0);
    let final_rgb = clamp(light_rgb + (ambient_color * ambient), vec3<f32>(0.0), vec3<f32>(1.0));
    c *= vec4f(final_rgb, 1.0);
    c.a = 1.0;



    // so this is a slightly different way of doing lighting
    // essentially just applying more light to areas that are already light (shitty specular)
    //let brightness = ((c.r + c.g + c.b) * (c.r + c.g + c.b)) / 1.0;
    //let ambient = c.rgb * 0.05; 
    //let light_contribution = light_rgb * c.rgb * brightness * 5.0;
    //c = vec4f(ambient + light_contribution, 1.0);


    return c;


}

fn unpack_rgbi(packed: u32) -> vec4<f32> {
    var r = f32((packed >> 24) & 0x000000FF);
    var g = f32((packed >> 16) & 0x000000FF);
    var b = f32((packed >> 8) & 0x000000FF);
    var intensity = f32(packed & 0x000000FF);
    return vec4f(r, g, b, intensity) / 255.0;
}

fn pack_rgbi(rgbi: vec4<f32>) -> u32 {
    var packed = u32(0);
    packed |= u32(clamp(rgbi.x * 255.0, 0.0, 255.0)) << 24;
    packed |= u32(clamp(rgbi.y * 255.0, 0.0, 255.0)) << 16;
    packed |= u32(clamp(rgbi.z * 255.0, 0.0, 255.0)) << 8;
    packed |= u32(clamp(rgbi.w * 255.0, 0.0, 255.0));
    return packed;
}


