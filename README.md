# Contact Projective Decals
Based on Alexander Sannikovs talk on the rendering techniques of Path of Exile 2

<img width="856" alt="Image of a bunch of decals being projected on top of a bunch of boxes and stuff" src="https://github.com/naasblod/bevy_decal_lab/assets/51246882/85b17493-2428-41a1-9b54-83bf192fdc0a">

### !WARNING!
* This is a work in progress
* Currently requires the camera angle to be fixed to avoid distortions, (check the example for a visual and use left_moue and scroll to see the effect)
* I might change up the api a bit. But I sort of like this simple api for my use case.

Open for any and all issues and pull requests

### Description
This uses a standard material extension shader and a Quad (now Rectangle) mesh to render decals on top of geometry. It smoothly fades instead of clips when the quad intersects with geometry (), It uses the difference between the depth of the fragment in the mesh (Quad, now Rectangle) and the depth for the fragment in the depth buffer as a depth texture for the parallax mapping technique (this is what causes the effect of the decal to conform to the surrounding geometry).

The depth test is disabled so that decals that intersect with other geometry can be smoothly faded instead of culled.

In it's current state I think it's usable for fixed angle cameras since the effect will be deterministic, if you were to change the view vector the parts of the decal that are offset by geometry will shift (this might be fixable).

## Bevy Compatibility
This is not up on crates.io right now. Use 
`bevy_contact_projective_decals = { git = "https://https://github.com/naasblod/bevy_contact_projective_decals.git", branch = "main" }`

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
    spatial_bundle: SpatialBundle::from_transform(Transform::default()),
    standard_material: materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("my_texture.png")),
        alpha_mode: AlphaMode::Blend,
        ..default()
    }),
    ..default()
});
```

### todos:
* maybe the api is very bad. I don't know.
* figure out if it's possible to not have the world offset shift when rotating the camera

Super big thanks to NiseVoid and Griffin

uv checker map from https://github.com/Arahnoid/UVChecker-map
