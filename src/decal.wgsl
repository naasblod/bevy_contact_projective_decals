#import bevy_pbr::{
    mesh_view_bindings::view,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    prepass_utils::prepass_depth,
    view_transformations::{depth_ndc_to_view_z},
    mesh_view_bindings as view_bindings,
    parallax_mapping::parallaxed_uv,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct CustomMaterial {
    center_pos: vec3<f32>,
}
@group(2) @binding(200)
var<uniform> custom_material: CustomMaterial;

@fragment
fn fragment(in: VertexOutput,
    @builtin(front_facing) is_front: bool) -> @location(0) vec4<f32> {

    let v_ray = view.world_position - in.world_position.xyz;

    // view vector
    let V = normalize(v_ray) ;
    let N = in.world_normal;
    let T = in.world_tangent.xyz;
    let B = in.world_tangent.w * cross(N, T);
    // Transform V from fragment to camera in world space to tangent space.
    let Vt = vec3(dot(V, T), dot(V, B), dot(V, N));

    let frag_depth = depth_ndc_to_view_z(in.position.z);
    let depth_pass_depth = depth_ndc_to_view_z(prepass_depth(in.position, 0u));

    let diff_depth = frag_depth - depth_pass_depth;
    let diff_depth_abs = abs(diff_depth);

    var uv = in.uv;
    uv = parallaxed_uv(
        diff_depth,
        1.0,
        0u,
        uv,
        // Flip the direction of Vt to go toward the surface to make the
        // parallax mapping algorithm easier to understand and reason
        // about.
        -Vt,
    );

    var new_in = in;
    new_in.uv = uv;

    var pbr_input = pbr_input_from_standard_material(new_in, is_front);
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);
    pbr_input.world_position = vec4(in.world_position.xyz + (V * diff_depth_abs), depth_pass_depth);

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    let fade_factor = 6.0;
    var alpha = min(clamp(((1.0 - diff_depth_abs * fade_factor)), 0.0, 1.0), out.color.a);

    return vec4(out.color.rgb, alpha);
}
