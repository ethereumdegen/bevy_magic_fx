

 
use crate::magic_fx::MagicFxBillboardTarget;
use crate::magic_fx_variant::MagicFxStyle;
use bevy::prelude::*;


pub(crate) fn magic_fx_beam_plugin(app: &mut App) {
    
    app
         
       	.add_systems(Update, (
       		update_magic_beam_scale,
       		//update_magic_beam_rotation

       		).chain())
       ;

}


#[derive(Component,Clone,Debug)]
pub struct MagicFxBeamComponent {

	pub end_point: Vec3


}	



fn update_magic_beam_scale(

 
    billboard_target_query: Query<Entity, With<MagicFxBillboardTarget> >,

   global_xform_query: Query<&GlobalTransform>,

   beam_comp_query: Query<&MagicFxBeamComponent>,

    mut magicfx_query: Query<(Entity, &mut Transform,   &MagicFxStyle, &Parent)  >

){

	let Some(billboard_target_entity) = billboard_target_query.get_single( ) .ok() else {return};


  for( magic_fx_instance_entity, mut magicfx_xform,    magic_fx_style, parent ) in magicfx_query.iter_mut(){

  			let Some(billboard_target_xform) = global_xform_query.get(billboard_target_entity).ok() else {continue};
  			
  			let Some(parent_global_xform) = global_xform_query.get(parent.get()).ok() else {continue};

			let Some(instance_global_xform) = global_xform_query.get(magic_fx_instance_entity).ok() else {continue};

  			 if magic_fx_style != &MagicFxStyle::Beam {
                continue;
            }

  			let Some(magic_fx_beam_comp) = beam_comp_query.get( parent.get() ).ok() else {continue};

  			let start_point  = parent_global_xform.translation();
  			let end_point  = magic_fx_beam_comp.end_point.clone();

  			let billboard_target_position = billboard_target_xform.translation();

        
         
	         // Calculate the direction vector from start to end point
	        let beam_direction = (end_point - start_point).normalize();

	        // Calculate the distance between start and end points for scaling
	        let beam_length = start_point.distance(end_point);

	        // Calculate the direction vector from the plane to the billboard target point
	        let target_direction = (billboard_target_position - start_point).normalize();

	        // Calculate the quaternion rotation to align the beam with the direction from start to end
	        let rotation_to_end = Quat::from_rotation_arc(Vec3::Y, beam_direction);

	        // Calculate the quaternion rotation to face the target direction (billboard effect)
	        // Assuming that `Vec3::Y` is the up vector of the plane, use the cross product to find the required rotation axis
	       // let rotation_to_face_target = Quat::from_rotation_arc(Vec3::Z, Vec3::new(target_direction.x, target_direction.z, 0.0).normalize());

	        // Update the transform
	        // Set the position to the start point
	        magicfx_xform.translation = Vec3::new(0.0,0.0,0.0);

	        // Set the rotation to align the plane with the direction from start to end
	        // Combine the rotations for beam direction and facing the target
	        magicfx_xform.rotation = rotation_to_end ;

	        // Scale the plane along the Z-axis to match the distance from start to end
	        magicfx_xform.scale = Vec3::new(1.0, beam_length, 1.0);

	        info!("beam xform {:?}",magicfx_xform);


        }       


}

fn update_magic_beam_rotation(

	//mut magic_fx_query: Query< ( &mut Transform , &MagicFxBeamComponent) > ,
    target_query: Query<&GlobalTransform, With<MagicFxBillboardTarget> >,

    mut magicfx_query: Query<(&mut Transform, &GlobalTransform, &MagicFxStyle), Without<MagicFxBillboardTarget> >

){

    //let target_xform = target_query.get_single().cloned() ;

    if let Some(target_xform ) = target_query.get_single().ok().cloned() {
    

      
      
        for( mut magicfx_xform,  magicfx_global_xform, magic_fx_style) in magicfx_query.iter_mut(){
        
            if magic_fx_style != &MagicFxStyle::Beam {
                continue;
            }
          
            let dir = (target_xform.translation() - magicfx_global_xform.translation()).normalize();

            // Calculate yaw and pitch
            let yaw = -dir.z.atan2(dir.x);
            let pitch = -dir.y.asin();

            // Create quaternion from yaw and pitch
            let yaw_rotation = Quat::from_rotation_y(yaw);
         //   let pitch_rotation = Quat::from_rotation_x(pitch);

            // Combine rotations
            magicfx_xform.rotation = yaw_rotation ; // * pitch_rotation;


        }       



    }


}