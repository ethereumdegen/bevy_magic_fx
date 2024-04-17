use bevy::prelude::*;

use crate::euler_transform::EulerTransform;

pub(crate) fn lerp_transforms(
    start: &Transform,
    end: &Transform,
    t: f32, // Interpolation factor (0.0 = start, 1.0 = end)
) -> Transform {
    // Lerp translation
    let translation = start.translation.lerp(end.translation, t);

  

    // Lerp scale
    let scale = start.scale.lerp(end.scale, t);


   // Slerp rotation using quaternions
    let rotation = if start.rotation.dot(end.rotation) < 0.0 {
        start.rotation.slerp(-end.rotation, t)
    } else {
        start.rotation.slerp(end.rotation, t)
    };

      // Slerp rotation
   //let rotation = start.rotation.slerp(end.rotation, t);

    // Construct the new interpolated Transform
    Transform {
        translation,
        rotation,
        scale,
    }
}


pub(crate) fn lerp_euler_transforms(
   start: &EulerTransform,
    end: &EulerTransform,
    t: f32, // Interpolation factor (0.0 = start, 1.0 = end)
    ) -> EulerTransform {

    let translation = start.translation.lerp(end.translation, t);
    let scale = start.scale.lerp(end.scale, t);

    let rotation = start.rotation.lerp(end.rotation, t);


    EulerTransform {
        translation,
        rotation,
        scale,
    }


}

pub(crate) fn lerp_colors(
   start: &Color,
    end: &Color,
    t: f32, // Interpolation factor (0.0 = start, 1.0 = end)
    ) -> Color {

    let r = start.r().lerp(end.r(), t);
    let g = start.g().lerp(end.g(), t);
    let b = start.b().lerp(end.b(), t);
    let a = start.a().lerp(end.a(), t);
    


    Color::rgba(r, g, b, a)  


}