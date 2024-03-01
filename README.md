# Contact Projective Decals
Based on Alexander Sannikovs talk on the rendering techniques of Path of Exile 2

<img width="856" alt="Image of a bunch of decals being projected on top of a bunch of boxes and stuff" src="https://github.com/naasblod/bevy_decal_lab/assets/51246882/85b17493-2428-41a1-9b54-83bf192fdc0a">

### !WARNING!
* This is a work in progress
* Due to the nature of this technique looking at the decal from very steep angles will cause distortion, this can be mitigated by creating textures that are bigger than the effect, maybe the actual bullet hole in your decal is 16x16px then your image could be 32x32px or something like that.

Open for any and all issues and pull requests

### Description
This uses a standard material extension shader and a Quad (now Rectangle) mesh to render decals on top of geometry. It smoothly fades instead of clips when the quad intersects with geometry. It uses the difference between the depth of the fragment in the mesh (Quad, now Rectangle) and the depth for the fragment in the depth buffer as a depth texture for the parallax mapping technique (this is what causes the effect of the decal to conform to the surrounding geometry).

The depth test is disabled so that decals that intersect with other geometry can be smoothly faded instead of culled.

## Bevy Compatibility
This is not up on crates.io right now. Use 
`bevy_contact_projective_decals = { git = "https://github.com/naasblod/bevy_contact_projective_decals.git", branch = "main" }`

| Bevy Version | Crate Version |
|--------------|---------------|
| `0.13`       | `None.None.None`       |

## Example

Setup:
```rs
use bevy_contact_projective_decals::{DecalPlugin};
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DecalPlugin))
        .run();
}
```

Spawning:
```rs
commands.spawn(DecalBundle {
    transform: Transform::from_xyz(x, y, z),
    decal_material: decal_materials.add(ExtendedMaterial::<
        StandardMaterial,
        DecalMaterial,
    > {
        base: StandardMaterial {
            base_color_texture: Some(asset_server.load("my_decal.png"))),
            base_color: color,
            alpha_mode: AlphaMode::Blend,
            ..default()
        },
        extension: DecalMaterial {
            depth_fade_factor: 8.0,
        },
    }),
    mesh: meshes.add(decal_mesh_quad(Vec2::splat(scale))),
    ..default()
});
```

Super big thanks to NiseVoid and Griffin

uv checker map from https://github.com/Arahnoid/UVChecker-map
