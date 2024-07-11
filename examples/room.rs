use bevy::{
    core_pipeline::prepass::DepthPrepass,
    pbr::{ExtendedMaterial, NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use bevy_contact_projective_decals::{decal_mesh_quad, DecalBundle, DecalMaterial, DecalPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DecalPlugin, PanOrbitCameraPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, move_camera)
        .run();
}

fn move_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x += 1.0 * time.delta_seconds();
        }
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut decal_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, DecalMaterial>>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        NotShadowCaster,
        NotShadowReceiver,
        PbrBundle {
            mesh: meshes.add(Rectangle::new(10.0, 10.0)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            )),
            ..default()
        },
    ));
    commands.spawn((
        NotShadowCaster,
        NotShadowReceiver,
        PbrBundle {
            mesh: meshes.add(Cuboid::new(5.0, 5.0, 0.2)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 0.0, -2.5),
            ..Default::default()
        },
    ));

    commands.spawn((
        NotShadowCaster,
        NotShadowReceiver,
        PbrBundle {
            mesh: meshes.add(Cuboid::new(5.0, 5.0, 0.2)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 0.0, 2.5),
            ..Default::default()
        },
    ));

    commands.spawn((
        NotShadowCaster,
        NotShadowReceiver,
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.2, 5.0, 5.0)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(-2.5, 0.0, 0.0),
            ..Default::default()
        },
    ));

    commands.spawn((
        NotShadowCaster,
        NotShadowReceiver,
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.2, 5.0, 5.0)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(2.5, 0.0, 0.0),
            ..Default::default()
        },
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000000.0,
            range: 60.0,
            radius: 60.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });

    commands.spawn(DecalBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        decal_material: decal_materials.add(ExtendedMaterial::<StandardMaterial, DecalMaterial> {
            base: StandardMaterial {
                base_color_texture: Some(asset_server.load("blast.png")),
                base_color: Color::Srgba(Srgba::RED),
                alpha_mode: AlphaMode::Blend,
                ..default()
            },
            extension: DecalMaterial {
                depth_fade_factor: 8.0,
            },
        }),
        mesh: meshes.add(decal_mesh_quad(Vec3::Y)),
        ..default()
    });

    commands.spawn(DecalBundle {
        transform: Transform::from_xyz(-2.4, 1.0, 0.0),
        decal_material: decal_materials.add(ExtendedMaterial::<StandardMaterial, DecalMaterial> {
            base: StandardMaterial {
                base_color_texture: Some(asset_server.load("blast.png")),
                base_color: Color::Srgba(Srgba::RED),
                alpha_mode: AlphaMode::Blend,
                ..default()
            },
            extension: DecalMaterial {
                depth_fade_factor: 8.0,
            },
        }),
        mesh: meshes.add(decal_mesh_quad(Vec3::X)),
        ..default()
    });

    commands.spawn(DecalBundle {
        transform: Transform::from_xyz(2.4, 1.0, 0.0),
        decal_material: decal_materials.add(ExtendedMaterial::<StandardMaterial, DecalMaterial> {
            base: StandardMaterial {
                base_color_texture: Some(asset_server.load("blast.png")),
                base_color: Color::Srgba(Srgba::RED),
                alpha_mode: AlphaMode::Blend,
                ..default()
            },
            extension: DecalMaterial {
                depth_fade_factor: 8.0,
            },
        }),
        mesh: meshes.add(decal_mesh_quad(-Vec3::X)),
        ..default()
    });

    commands.spawn(DecalBundle {
        transform: Transform::from_xyz(0.0, 1.0, 2.4),
        decal_material: decal_materials.add(ExtendedMaterial::<StandardMaterial, DecalMaterial> {
            base: StandardMaterial {
                base_color_texture: Some(asset_server.load("blast.png")),
                base_color: Color::Srgba(Srgba::RED),
                alpha_mode: AlphaMode::Blend,
                ..default()
            },
            extension: DecalMaterial {
                depth_fade_factor: 8.0,
            },
        }),
        mesh: meshes.add(decal_mesh_quad(-Vec3::Z)),
        ..default()
    });

    commands.spawn(DecalBundle {
        transform: Transform::from_xyz(0.0, 1.0, -2.4),
        decal_material: decal_materials.add(ExtendedMaterial::<StandardMaterial, DecalMaterial> {
            base: StandardMaterial {
                base_color_texture: Some(asset_server.load("blast.png")),
                base_color: Color::Srgba(Srgba::RED),
                alpha_mode: AlphaMode::Blend,
                ..default()
            },
            extension: DecalMaterial {
                depth_fade_factor: 8.0,
            },
        }),
        mesh: meshes.add(decal_mesh_quad(Vec3::Z)),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(1.0, 6.0, 0.0)
                .looking_at(Vec3::new(0.0, 6.0, 0.0), Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
        DepthPrepass,
    ));
}
