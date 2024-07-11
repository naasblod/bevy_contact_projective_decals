use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_screen_diagnostics::{
    Aggregate, ScreenDiagnostics, ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin,
};
use std::time::Duration;

use bevy::{
    core_pipeline::prepass::DepthPrepass,
    diagnostic::{Diagnostic, DiagnosticPath, Diagnostics, RegisterDiagnostic},
    pbr::ExtendedMaterial,
    prelude::*,
};
use bevy_contact_projective_decals::{decal_mesh_quad, DecalBundle, DecalMaterial, DecalPlugin};
use rand::{thread_rng, Rng};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DecalPlugin,
            PanOrbitCameraPlugin,
            EguiPlugin,
            ScreenDiagnosticsPlugin::default(),
            ScreenFrameDiagnosticsPlugin,
        ))
        .register_diagnostic(Diagnostic::new(DECAL_COUNT))
        .add_systems(Startup, (setup, setup_diagnostic))
        .add_systems(Update, (spawn_decals, thing_count))
        .run();
}
#[derive(Resource)]
struct DecalHandles {
    mat_uv: Handle<ExtendedMaterial<StandardMaterial, DecalMaterial>>,
    mesh: Handle<Mesh>,
}

#[allow(clippy::too_many_arguments)]
fn spawn_decals(
    mut commands: Commands,
    mut local_timer: Local<Timer>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut spawn: Local<bool>,
    my_handles: Res<DecalHandles>,
) {
    if local_timer.duration() == Duration::ZERO {
        local_timer.set_duration(Duration::from_secs_f32(0.01));
        local_timer.set_mode(TimerMode::Repeating);
    }
    if input.just_pressed(KeyCode::Space) {
        *spawn = !*spawn;
    }
    if !*spawn {
        return;
    }
    local_timer.tick(time.delta());

    for _ in 0..local_timer.times_finished_this_tick() {
        let x = thread_rng().gen_range(-5.0..5.0);
        let z = thread_rng().gen_range(-5.0..5.0);
        let scale = thread_rng().gen();
        commands.spawn(DecalBundle {
            transform: Transform::from_xyz(x, 0.0, z).with_scale(Vec3::splat(scale)),
            decal_material: my_handles.mat_uv.clone(),
            mesh: my_handles.mesh.clone(),
            ..default()
        });
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
    let decal_material = decal_materials.add(ExtendedMaterial::<StandardMaterial, DecalMaterial> {
        base: StandardMaterial {
            base_color_texture: Some(asset_server.load("UVCheckerMap01-512.png")),
            base_color: Color::WHITE,
            alpha_mode: AlphaMode::Blend,
            ..default()
        },
        extension: DecalMaterial {
            depth_fade_factor: 8.0,
        },
    });
    commands.insert_resource(DecalHandles {
        mat_uv: decal_material,
        mesh: meshes.add(decal_mesh_quad(Vec3::Y)),
    });
    commands.spawn((TextBundle::from_section(
        "Press space to start / stop.\nDrag mouse to pan, scroll to zoom",
        TextStyle {
            font_size: 25.,
            ..default()
        },
    ),));
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
            intensity: 10000000.0,
            range: 60.0,
            radius: 60.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(14.0, 8.0, 4.0),
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
const DECAL_COUNT: DiagnosticPath = DiagnosticPath::const_new("decal_count");
fn setup_diagnostic(mut onscreen: ResMut<ScreenDiagnostics>) {
    onscreen
        .add("decals".to_string(), DECAL_COUNT)
        .aggregate(Aggregate::Value)
        .format(|v| format!("{v:.0}"));
}

fn thing_count(
    mut diagnostics: Diagnostics,
    parts: Query<&Handle<ExtendedMaterial<StandardMaterial, DecalMaterial>>>,
) {
    diagnostics.add_measurement(&DECAL_COUNT, || parts.iter().len() as f64);
}
