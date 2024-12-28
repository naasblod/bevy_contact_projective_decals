//! ## Example
//!
//! Setup:
//! ```rs
//! use bevy_contact_projective_decals::{DecalPlugin};
//! fn main() {
//!     App::new()
//!         .add_plugins((DefaultPlugins, DecalPlugin))
//!         .run();
//! }
//! ```
//!
//! Spawning:
//! ```rs
//! commands.spawn(DecalBundle {
//!     transform: Transform::from_xyz(x, y, z),
//!     decal_material: decal_materials.add(ExtendedMaterial::<
//!         StandardMaterial,
//!         DecalMaterial,
//!     > {
//!         base: StandardMaterial {
//!             base_color_texture: Some(asset_server.load("my_decal.png"))),
//!             base_color: color,
//!             alpha_mode: AlphaMode::Blend,
//!             ..default()
//!         },
//!         extension: DecalMaterial {
//!             depth_fade_factor: 8.0,
//!         },
//!     }),
//!     mesh: meshes.add(decal_mesh_quad(Vec2::splat(scale))),
//!     ..default()
//! });
//! ```
mod decal;
pub use decal::{decal_mesh_quad, DecalMaterialExtension, DecalMeshMaterial3d, DecalMaterial, DecalPlugin};
