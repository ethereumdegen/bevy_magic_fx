use std::time::Duration;

use bevy::prelude::*;

use crate::{
    magic_fx_variant::{MagicFxInstance, MagicFxVariant},
    util::lerp_transforms,
};

#[derive(Component)]
pub struct MagicFxVariantComponent {
    pub magic_fx: MagicFxVariant,
    pub start_time: Duration,
}



#[derive(Component)]
pub struct MagicFxBillboardTarget {
     
}


#[derive(Component)]
pub struct MagicFxBillboardComponent {
     
}

#[derive(Component)]
pub struct MagicFxInstanceComponent {
    pub instance: MagicFxInstance,
  //  pub start_time: Duration, // find from parent 
}

/*

build plugins for the above comps

*/

pub fn update_magic_fx_variants_added(
    mut commands: Commands,
    magic_fx_query: Query<(Entity, &MagicFxVariantComponent), Added<MagicFxVariantComponent>>,
    //time: Res<Time>,
) {
    for (fx_entity, magic_fx_comp) in magic_fx_query.iter() {
        let magic_fx = &magic_fx_comp.magic_fx;

        for instance in magic_fx.magic_fx_instances.iter() {
            //println!("spawn magic fx instance!!");

            let bundle = &instance.to_bundle();

            let magic_fx_child = commands
                .spawn((
                    bundle.clone(),
                    MagicFxInstanceComponent {
                        instance: instance.clone(),
                       // start_time: time.elapsed(),
                    },
                    bevy::pbr::NotShadowCaster,
                ))
                .id();

            commands.entity(fx_entity).add_child(magic_fx_child);

            if *&instance.billboard_mesh {
                commands.entity(magic_fx_child).insert(MagicFxBillboardComponent{});
            }
        }
    }
}

pub fn update_magic_fx_variants(
    mut commands: Commands,
    mut magic_fx_query: Query<(Entity, &mut MagicFxVariantComponent)>,
    time: Res<Time>,
) {
    let current_time = time.elapsed();

    for (fx_entity, mut magic_fx_comp) in magic_fx_query.iter_mut() {
        let magic_fx = &magic_fx_comp.magic_fx;

       // let repeating = magic_fx.repeating;
       
            if current_time > magic_fx_comp.start_time + magic_fx.max_time_offset {
                
                 if magic_fx.repeating {
                    //reset 
                     magic_fx_comp.start_time = current_time;
                     
                     }else { 
                    commands.entity(fx_entity).despawn_recursive();
                }
            }
        
    }
}

pub fn update_magic_fx_instances(
    mut magic_fx_instance_query: Query<(
        Entity,
        &mut Visibility,
        &mut Transform,
        &MagicFxInstanceComponent,
        &Parent
    )>,

    magic_fx_variant_query: Query<&MagicFxVariantComponent>,
    time: Res<Time>,
) {
    let current_time = time.elapsed();

    for (entity, mut fx_visibility, mut fx_xform, instance_comp, parent) in magic_fx_instance_query.iter_mut() {
       
        let Some(magic_fx_variant) = magic_fx_variant_query.get( parent.get()  ).ok() else {continue};


        let instance = &instance_comp.instance;
        let is_billboarded = &instance.billboard_mesh;
        let start_time = magic_fx_variant.start_time + instance.start_time_offset;
        let end_time = magic_fx_variant.start_time + instance.end_time_offset;

      

        let is_visible = current_time >= start_time && current_time <= end_time;

        *fx_visibility = match is_visible {
            true => Visibility::Inherited,
            false => Visibility::Hidden,
        };

        //if not billboarded, that means rotation should occur 
       

            let start_xform = instance.start_transform;
            let end_xform = instance.end_transform;

            if is_visible {
                let duration = end_time.as_secs_f32() - start_time.as_secs_f32();
                let elapsed = current_time.as_secs_f32() - start_time.as_secs_f32();

                let lerp_amount: f32 = if duration > 0.0 {
                    elapsed / duration
                } else {
                    0.0
                }
                .clamp(0.0, 1.0);

                // Assuming `Transform` struct has a `lerp` function
                 let new_transform =  lerp_transforms(&start_xform, &end_xform, lerp_amount);


                 fx_xform.translation = new_transform.translation;
                 fx_xform.scale = new_transform.scale;

                  if !is_billboarded {
                    fx_xform.rotation = new_transform.rotation;
                  }

            } else {
                // If not visible, or outside the lerp range, you might want to set it to start or end transform
                // Adjust this based on your needs, for now, let's assume it resets to start_xform if not visible
                let new_transform = start_xform;

                 fx_xform.translation = new_transform.translation;
                 fx_xform.scale = new_transform.scale;

                  if !is_billboarded {
                    fx_xform.rotation = new_transform.rotation;
                  }



               // *fx_xform = start_xform;
            }

          

       // println!("update visibility {:?}", is_visible);
        //let bundle =  &mut instance.to_bundle() ;
    }
}




pub fn update_magicfx_billboard_rotation(

    target_query: Query<&Transform, (With<MagicFxBillboardTarget>, Without<MagicFxBillboardComponent>)>,

    mut magicfx_billboard_query: Query<&mut Transform, (With<MagicFxBillboardComponent>,Without<MagicFxBillboardTarget>)>

){

    let target_xform = target_query.get_single().cloned().unwrap_or(Transform::from_xyz(0.0,0.0,0.0));
 

    //this mostly works but is a bit weird ... 

    for mut magicfx_xform in magicfx_billboard_query.iter_mut(){

            // Calculate the direction from the billboarded object to the camera
            let direction = target_xform.translation - magicfx_xform.translation;

            // Calculate the rotation needed to face the camera
            let look_at_rotation = Quat::from_rotation_arc(Vec3::Z, direction.normalize());

            // Update the rotation of the billboarded object
            magicfx_xform.rotation = look_at_rotation;

    }



 




}