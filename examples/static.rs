use bevy::{core_pipeline::prepass::DepthPrepass, pbr::ExtendedMaterial, prelude::*};
use bevy_contact_projective_decals::{decal_mesh_quad, DecalBundle, DecalMaterial, DecalPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use rand::{thread_rng, Rng};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DecalPlugin,
            PanOrbitCameraPlugin,
            WorldInspectorPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_light_type, move_camera))
        .run();
}
enum LightType {
    Point,
    Spot,
}
#[derive(Component)]
struct MyLight(pub LightType);

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

fn toggle_light_type(
    mut commands: Commands,
    query: Query<(Entity, &MyLight)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let (entity, light) = query.single();
        match light.0 {
            LightType::Point => {
                commands.spawn((
                    SpotLightBundle {
                        spot_light: SpotLight {
                            shadows_enabled: true,
                            intensity: 5000000.0,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(0.0, 1.8, 4.0),
                        ..Default::default()
                    },
                    MyLight(LightType::Spot),
                ));
            }
            LightType::Spot => {
                commands.spawn((
                    PointLightBundle {
                        point_light: PointLight {
                            intensity: 1000000.0,
                            range: 60.0,
                            radius: 60.0,
                            shadows_enabled: true,
                            ..default()
                        },
                        transform: Transform::from_xyz(2.0, 4.0, 4.0),
                        ..default()
                    },
                    MyLight(LightType::Point),
                ));
            }
        };

        commands.entity(entity).despawn_recursive();
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

    commands
        .spawn((
            Name::new("root"),
            NodeBundle {
                background_color: BackgroundColor(Color::NONE),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::End,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|root_children| {
            root_children.spawn(TextBundle::from_section(
                "Press space to change lighting.",
                TextStyle {
                    font_size: 24.0,
                    color: Color::Srgba(Srgba::WHITE),
                    ..default()
                },
            ));
        });

    commands.spawn((
        MyLight(LightType::Spot),
        SpotLightBundle {
            spot_light: SpotLight {
                shadows_enabled: true,
                intensity: 5000000.0,
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 1.8, 4.0),
            ..Default::default()
        },
    ));

    commands.spawn(DecalBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(4.0)),
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
