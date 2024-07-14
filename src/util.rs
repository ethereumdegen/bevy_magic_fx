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
   start: &LinearRgba,
    end: &LinearRgba,
    t: f32, // Interpolation factor (0.0 = start, 1.0 = end)
    ) -> LinearRgba {



    let r = start.red.lerp(end.red, t);
    let g = start.green.lerp(end.green, t);
    let b = start.blue.lerp(end.blue, t);
    let a = start.alpha.lerp(end.alpha, t);
    


    Color::srgba(r, g, b, a).into()  


}