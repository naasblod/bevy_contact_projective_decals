use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use std::time::Duration;

use bevy::{core_pipeline::prepass::DepthPrepass, prelude::*};
use bevy_contact_projective_decals::{Decal, DecalBundle, DecalPlugin};
use rand::{seq::SliceRandom, thread_rng, Rng};
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DecalPlugin, PanOrbitCameraPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_decals.after(decal_cleanup), decal_cleanup))
        .run();
}

fn spawn_decals(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut local_timer: Local<Timer>,
    time: Res<Time>,
) {
    if local_timer.duration() == Duration::ZERO {
        local_timer.set_duration(Duration::from_secs_f32(0.01));
        local_timer.set_mode(TimerMode::Repeating);
    }
    local_timer.tick(time.delta());

    if local_timer.finished() {
        let x = thread_rng().gen_range(-5.0..5.0);
        let z = thread_rng().gen_range(-5.0..5.0);
        let decal_str =
            ["boys.png", "blast.png", "UVCheckerMap01-512.png"].choose(&mut thread_rng());
        let (r, g, b) = thread_rng().gen();

        let color = if *decal_str.unwrap() == "UVCheckerMap01-512.png" {
            Color::WHITE
        } else {
            Color::rgb_from_array((r, g, b))
        };
        let scale = thread_rng().gen();
        commands.spawn(DecalBundle {
            transform: Transform::from_xyz(x, 0.0, z).with_scale(Vec3::splat(scale)),
            standard_material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load(*decal_str.unwrap())),
                base_color: color,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            ..default()
        });
    }
}
fn decal_cleanup(mut commands: Commands, query: Query<Entity, With<Decal>>) {
    let max = 1000000;
    let mut amount_to_delete: i32 = query.iter().len() as i32 - max;

    while amount_to_delete > 0 {
        let index = thread_rng().gen_range(0..(max - 1));

        let entity = query.iter().collect::<Vec<Entity>>()[index as usize];
        commands.entity(entity).despawn_recursive();
        amount_to_delete -= 1;
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
