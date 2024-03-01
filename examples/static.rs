use bevy::{core_pipeline::prepass::DepthPrepass, prelude::*};
use bevy_contact_projective_decals::{DecalBundle, DecalPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use rand::{thread_rng, Rng};
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DecalPlugin, PanOrbitCameraPlugin))
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
    asset_server: Res<AssetServer>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Rectangle::new(10.0, 10.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    let num_obs = 10;
    for i in 0..num_obs {
        for j in 0..num_obs {
            let rotation_axis: [f32; 3] = thread_rng().gen();
            let rotation_vec: Vec3 = rotation_axis.into();
            let rotation: u32 = thread_rng().gen_range(0..360);
            let transform = Transform::from_xyz(
                (-num_obs + 1) as f32 / 2.0 + i as f32,
                -0.2,
                (-num_obs + 1) as f32 / 2.0 + j as f32,
            )
            .with_rotation(Quat::from_axis_angle(
                rotation_vec.normalize_or_zero(),
                (rotation as f32).to_radians(),
            ));
            commands.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.6, 0.6, 0.6)),
                material: materials.add(Color::WHITE),
                transform,
                ..default()
            });
        }
    }
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000000.0,
            range: 60.0,
            radius: 60.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 4.0, 4.0),
        ..default()
    });

    commands.spawn(DecalBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        standard_material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("blast.png")),
            base_color: Color::RED,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        ..default()
    });
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(2.0, 9.5, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
        DepthPrepass,
    ));
}
