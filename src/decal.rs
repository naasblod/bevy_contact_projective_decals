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
        )
        .add_systems(Startup, setup_mesh_handle)
        .add_systems(Update, create_decals);
    }
}

#[derive(Resource)]
struct MeshHandle {
    quad: Handle<Mesh>,
}

fn setup_mesh_handle(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mut mesh: Mesh = Rectangle::new(1.0, 1.0).into();
    mesh = mesh
        .with_generated_tangents()
        .unwrap()
        .rotated_by(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));
    let mesh_handle = meshes.add(mesh);
    commands.insert_resource(MeshHandle { quad: mesh_handle });
}

#[derive(Bundle, Default)]
pub struct DecalBundle {
    pub spatial_bundle: SpatialBundle,
    pub standard_material: Handle<StandardMaterial>,
    pub decal: Decal,
    pub not_shadow_caster: NotShadowCaster,
}

#[derive(Component, Default)]
pub struct Decal;

fn create_decals(
    mut commands: Commands,
    query: Query<(Entity, &Handle<StandardMaterial>), Added<Decal>>,
    materials: Res<Assets<StandardMaterial>>,
    meshes: Res<MeshHandle>,
    mut decal_extended_material: ResMut<Assets<ExtendedMaterial<StandardMaterial, DecalMaterial>>>,
) {
    for (entity, standard_material) in &query {
        if let Some(material) = materials.get(standard_material) {
            let decal_handle =
                decal_extended_material.add(ExtendedMaterial::<StandardMaterial, DecalMaterial> {
                    base: material.clone(),
                    extension: DecalMaterial {
                        depth_fade_factor: 8.0,
                    },
                });
            commands
                .entity(entity)
                .insert(meshes.quad.clone())
                .remove::<Handle<StandardMaterial>>()
                .insert(decal_handle);
        }
    }
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
