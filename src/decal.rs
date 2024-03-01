use bevy::{
    asset::embedded_asset,
    math::primitives::Rectangle,
    pbr::{
        ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline,
        NotShadowCaster,
    },
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, CompareFunction, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};

pub struct DecalPlugin;
impl Plugin for DecalPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "decal.wgsl");
        app.add_plugins(
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, DecalMaterial>> {
                prepass_enabled: false,
                ..default()
            },
        );
    }
}

pub fn decal_mesh_quad(size: Vec2) -> Mesh {
    let mut mesh: Mesh = Rectangle::from_size(size).into();
    mesh = mesh
        .with_generated_tangents()
        .unwrap()
        .rotated_by(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));
    mesh
}

#[derive(Bundle, Default)]
pub struct DecalBundle {
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub decal_material: Handle<ExtendedMaterial<StandardMaterial, DecalMaterial>>,
    pub mesh: Handle<Mesh>,
    pub not_shadow_caster: NotShadowCaster,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl MaterialExtension for DecalMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_contact_projective_decals/decal.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // descriptor.primitive.cull_mode = None;
        if let Some(label) = &mut descriptor.label {
            *label = format!("decal_{}", *label).into();
        }
        if let Some(ref mut depth) = &mut descriptor.depth_stencil {
            depth.depth_compare = CompareFunction::Always;
        }

        Ok(())
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct DecalMaterial {
    #[uniform(200)]
    pub depth_fade_factor: f32,
}
impl Default for DecalMaterial {
    fn default() -> Self {
        Self {
            depth_fade_factor: 8.0,
        }
    }
}
