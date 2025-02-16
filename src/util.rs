use bevy::prelude::*;

use crate::euler_transform::EulerTransform;

/*
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
}*/


pub(crate) fn lerp_euler_transforms(
   start: &EulerTransform,
    end: &EulerTransform,
    t: f32, // Interpolation factor (0.0 = start, 1.0 = end)

    ease_function_type: EaseFunction, 

    ) -> EulerTransform {

    //let translation = start.translation.lerp(end.translation, t);
    //let scale = start.scale.lerp(end.scale, t);
    //let rotation = start.rotation.lerp(end.rotation, t);


    let translation = lerp_vec3(&start.translation, &end.translation, t , ease_function_type) ;
    let scale = lerp_vec3(&start.scale, &end.scale, t , ease_function_type) ;
    let rotation = lerp_vec3(&start.rotation, &end.rotation, t , ease_function_type) ;


    EulerTransform {
        translation,
        rotation,
        scale,
    }


}



fn lerp_vec3( start:&Vec3, end: &Vec3, t:f32, ease_function_type: EaseFunction) -> Vec3 {


    let x = EasingCurve::new(start.x, end.x, ease_function_type).sample_clamped(t);
    let y = EasingCurve::new(start.y, end.y, ease_function_type).sample_clamped(t);
    let z = EasingCurve::new(start.z, end.z, ease_function_type).sample_clamped(t);

    Vec3::new(x, y, z)

}




pub(crate) fn lerp_colors(
    start: &Color,
    end: &Color,
    t: f32, // Interpolation factor (0.0 = start, 1.0 = end)

     ease_function_type: EaseFunction, 

    ) -> Color {



    //interpolate linearly ? 
    let start = start.to_linear();
    let end = end.to_linear();

    let r = EasingCurve::new(start.red, end.red, ease_function_type).sample_clamped(t);
    let g = EasingCurve::new(start.green, end.green, ease_function_type).sample_clamped(t);
    let b = EasingCurve::new(start.blue, end.blue, ease_function_type).sample_clamped(t);
    let a = EasingCurve::new(start.alpha, end.alpha, ease_function_type).sample_clamped(t);

  

    Color::LinearRgba(LinearRgba { red: r, green: g, blue: b, alpha: a })
   // Color::rgba(r, g, b, a).into()  


}