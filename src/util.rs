use bevy::prelude::*;

pub(crate) fn lerp_transforms(
    start: &Transform,
    end: &Transform,
    t: f32, // Interpolation factor (0.0 = start, 1.0 = end)
) -> Transform {
    // Lerp translation
    let translation = start.translation.lerp(end.translation, t);

    // Slerp rotation
    let rotation = start.rotation.slerp(end.rotation, t);

    // Lerp scale
    let scale = start.scale.lerp(end.scale, t);

    // Construct the new interpolated Transform
    Transform {
        translation,
        rotation,
        scale,
    }
}
