
@group(0) @binding(0)
var<storage, read_write> buffer: array<u32>;
@group(0) @binding(1)
var<uniform> current_frame: u32;
@group(0) @binding(2)
var<uniform> total_frames: u32;
@group(0) @binding(3)
var<uniform> buffer_size: u32;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) gid : vec3<u32>) {

    let coords = get_buffer_coords(gid.xy, current_frame, total_frames, buffer_size);
    let idx = coords.x + coords.y * buffer_size;
    if (idx < buffer_size * buffer_size) {
        buffer[idx] = 0;
    }
}



fn get_buffer_coords(
    gid: vec2<u32>,
    current_frame: u32,
    total_frames: u32,
    buffer_size: u32
) -> vec2<u32> {
    let slice_height = buffer_size / total_frames;
    let start_y = current_frame * slice_height;
    let global_y = start_y + gid.y;


    return vec2<u32>(gid.x, global_y);
}