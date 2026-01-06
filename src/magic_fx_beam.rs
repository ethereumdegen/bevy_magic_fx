

 
use bevy::ecs::relationship::Relationship;
use crate::magic_fx::MagixFxStandardUpdateSet;
use crate::magic_fx::MagicFxBillboardTarget;
use crate::magic_fx_variant::MagicFxStyle;
use bevy::prelude::*;


pub(crate) fn magic_fx_beam_plugin(app: &mut App) {
    
    app
         
       	.add_systems(Update, (
       		update_magic_beam_xform,
       		//update_magic_beam_rotation

       		).chain()
       	.after( MagixFxStandardUpdateSet )

       	)
       ;

}


#[derive(Component,Clone,Debug,Default)]
pub struct MagicFxBeamComponent {

	pub end_point: Option<Vec3>


}	



fn update_magic_beam_xform(

 
   billboard_target_query: Query<Entity, With<MagicFxBillboardTarget> >,

   global_xform_query: Query<&GlobalTransform>,

   beam_comp_query: Query<&MagicFxBeamComponent>,

    mut magicfx_query: Query<(Entity, &mut Transform,   &MagicFxStyle, &ChildOf)  >

){

	let Some(billboard_target_entity) = billboard_target_query.single( ) .ok() else {return};


  for( magic_fx_instance_entity, mut magicfx_xform,    magic_fx_style, parent ) in magicfx_query.iter_mut(){

  			let Some(billboard_target_xform) = global_xform_query.get(billboard_target_entity).ok() else {continue};
  			
  			let Some(parent_global_xform) = global_xform_query.get(parent.get()).ok() else {continue};

			let Some(instance_global_xform) = global_xform_query.get(magic_fx_instance_entity).ok() else {continue};

  			 if magic_fx_style != &MagicFxStyle::Beam {
                continue;
            }

  			let Some(magic_fx_beam_comp) = beam_comp_query.get( parent.get() ).ok() else {continue};

  			let start_point  = parent_global_xform.translation();
  			let Some(end_point)  = magic_fx_beam_comp.end_point.clone() else{continue};

  			let billboard_target_position = billboard_target_xform.translation();

        
         
	         // Calculate the direction vector from start to end point
	        let beam_direction = (end_point - start_point).normalize();

	        // Calculate the distance between start and end points for scaling
	        let beam_length = start_point.distance(end_point);

	        // Calculate the direction vector from the plane to the billboard target point
	        let target_direction = (billboard_target_position - start_point).normalize();

	        // Calculate the quaternion rotation to align the beam with the direction from start to end
	        let rotation_to_end = Quat::from_rotation_arc(Vec3::Y, beam_direction);

	               // Calculate the horizontal direction to the target
	        let horizontal_target_direction = Vec3::new(target_direction.x, 0.0, target_direction.z).normalize();

	        // Calculate the rotation to face the target on the XZ plane (around Y-axis)
	        let current_beam_direction = Vec3::new(beam_direction.x, 0.0, beam_direction.z).normalize();
	        let rotation_to_face_billboard_target = Quat::from_rotation_arc(current_beam_direction, horizontal_target_direction);



 
	        // Update the transform
	        // Set the position to the start point
	        magicfx_xform.translation = Vec3::new(0.0,0.0,0.0);

	        // Set the rotation to align the plane with the direction from start to end
	        // Combine the rotations for beam direction and facing the target
	        magicfx_xform.rotation = rotation_to_end * rotation_to_face_billboard_target;

	        // Scale the plane along the Z-axis to match the distance from start to end
	       // magicfx_xform.scale = Vec3::new(1.0, beam_length, 1.0);

	        magicfx_xform.scale.y = beam_length; 

	        //info!("beam xform {:?}",magicfx_xform);


        }       


} 