
@group(0) @binding(0)
var<uniform> uni: f32;


@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    //my_storage[invocation_id.x] = my_storage[invocation_id.x] + uni;
    let _u = uni;
}



