#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip, mesh_position_local_to_world}
#import bevy_render::maths::affine_to_square;
#import bevy_pbr::mesh_view_bindings::view
#import bevy_pbr::mesh_types::Mesh

@group(1) @binding(0)
var<uniform> mesh: Mesh;

struct Vertex {
    //@builtin(instance_index) instance_index: u32,

    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    // Instance Data
    @location(3) i_pos_scale: vec4<f32>,
    @location(4) i_color: vec4<f32>,
    @location(5) instance_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let position = vertex.position * vertex.i_pos_scale.w + vertex.i_pos_scale.xyz;
    //Working:
    //let world_position = get_world_from_local(2u) * vec4<f32>(position, 1.0);
    let world_position = mesh_position_local_to_world(get_world_from_local(vertex.instance_index), vec4<f32>(position, 1.0));

    //let world_position = mesh_position_local_to_world(get_world_from_local(vertex.instance_index), vec4<f32>(position, 1.0));
    //let world_position = affine_to_square(mesh.model) * vec4<f32>(position, 1.0);

    var out: VertexOutput;

    // out.clip_position = mesh_position_local_to_clip(
    //     get_world_from_local(2u),
    //     vec4<f32>(vertex.position, 1.0),
    // );
    out.clip_position = view.clip_from_world * world_position;

    
    // NOTE: Passing 0 as the instance_index to get_model_matrix() is a hack
    // for this example as the instance_index builtin would map to the wrong
    // index in the Mesh array. This index could be passed in via another
    // uniform instead but it's unnecessary for the example.
    //out.clip_position = mesh_position_local_to_clip(
    //    get_model_matrix(1u),
    //    vec4<f32>(position, 1.0)
    //);
    out.color = vertex.i_color;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
