use std::ops::Mul;
use std::{ops::Div, time::Duration};

use bevy::prelude::*;

use crate::{
    animated_material::{AnimatedMaterial, AnimatedMaterialBase}, 
    magic_fx_variant::{MagicFxInstance, MagicFxVariant},
     util::{lerp_euler_transforms,lerp_colors}
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
pub struct MagicFxStandardRotationComponent {
     
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
            }else{
                 commands.entity(magic_fx_child).insert(MagicFxStandardRotationComponent{});
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

pub fn update_magic_fx_instances_visibility(
    mut magic_fx_instance_query: Query<(
        Entity,
        &mut Visibility,
        
        &MagicFxInstanceComponent,
        &Parent
    )>,

    magic_fx_variant_query: Query<&MagicFxVariantComponent>,
    time: Res<Time>,
) {
    let current_time = time.elapsed();

    for (entity, mut fx_visibility, instance_comp, parent) in magic_fx_instance_query.iter_mut() {
       
        let Some(magic_fx_variant) = magic_fx_variant_query.get( parent.get()  ).ok() else {continue};


        let instance = &instance_comp.instance;
       // let is_billboarded = &instance.billboard_mesh;
        let start_time = magic_fx_variant.start_time + instance.start_time_offset;
        let end_time = magic_fx_variant.start_time + instance.end_time_offset;

      

        let is_visible = current_time >= start_time && current_time <= end_time;

        *fx_visibility = match is_visible {
            true => Visibility::Inherited,
            false => Visibility::Hidden,
        };

        //if not billboarded, that means rotation should occur 
       

           

          

       // println!("update visibility {:?}", is_visible);
        //let bundle =  &mut instance.to_bundle() ;
    }
}


pub fn update_magic_fx_instances_translation_scale(
    mut magic_fx_instance_query: Query<(
        Entity,
         
        &mut Transform,
        &MagicFxInstanceComponent,
        &Parent
    )>,

    magic_fx_variant_query: Query<&MagicFxVariantComponent>,
    time: Res<Time>,
) {
    let current_time = time.elapsed();

    for (entity,   mut fx_xform, instance_comp, parent) in magic_fx_instance_query.iter_mut() {
       
        let Some(magic_fx_variant) = magic_fx_variant_query.get( parent.get()  ).ok() else {continue};


        let instance = &instance_comp.instance;
        //let is_billboarded = &instance.billboard_mesh;
        let start_time = magic_fx_variant.start_time + instance.start_time_offset;
        let end_time = magic_fx_variant.start_time + instance.end_time_offset;

      

        let is_visible = current_time >= start_time && current_time <= end_time;
 
        //if not billboarded, that means rotation should occur 
       

            let start_xform = instance.start_transform.clone();
            let end_xform = instance.end_transform.clone();

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
                 let new_transform =  lerp_euler_transforms(&start_xform, &end_xform, lerp_amount).to_transform();


                 fx_xform.translation = new_transform.translation;
                 fx_xform.scale = new_transform.scale;

                 // if !is_billboarded {
                 //   fx_xform.rotation = new_transform.rotation;
                 // }

            } else {
                // If not visible, or outside the lerp range, you might want to set it to start or end transform
                // Adjust this based on your needs, for now, let's assume it resets to start_xform if not visible
                let new_transform = start_xform.to_transform();

                 fx_xform.translation = new_transform.translation;
                 fx_xform.scale = new_transform.scale;

                  //if !is_billboarded {
                  //  fx_xform.rotation = new_transform.rotation;
                  //}



               // *fx_xform = start_xform;
            }

          

       // println!("update visibility {:?}", is_visible);
        //let bundle =  &mut instance.to_bundle() ;
    }
}


pub fn update_magicfx_standard_rotation(
    mut magic_fx_instance_query: Query<(
        Entity,
         
        &mut Transform,
        &MagicFxInstanceComponent,
        &Parent
    ), With<MagicFxStandardRotationComponent> >,

    magic_fx_variant_query: Query<&MagicFxVariantComponent>,
    time: Res<Time>,
) {
    let current_time = time.elapsed();

    for (entity,   mut fx_xform, instance_comp, parent) in magic_fx_instance_query.iter_mut() {
       
        let Some(magic_fx_variant) = magic_fx_variant_query.get( parent.get()  ).ok() else {continue};


        let instance = &instance_comp.instance;
        
        let start_time = magic_fx_variant.start_time + instance.start_time_offset;
        let end_time = magic_fx_variant.start_time + instance.end_time_offset;

      

        let is_visible = current_time >= start_time && current_time <= end_time;
 
         

            let start_xform = instance.start_transform.clone();
            let end_xform = instance.end_transform.clone();

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
                 let new_transform =  lerp_euler_transforms(&start_xform, &end_xform, lerp_amount).to_transform();

 
                  
                    fx_xform.rotation = new_transform.rotation;
                  

            } else {
                // If not visible, or outside the lerp range, you might want to set it to start or end transform
                // Adjust this based on your needs, for now, let's assume it resets to start_xform if not visible
                let new_transform = start_xform.to_transform();

                
               
                    fx_xform.rotation = new_transform.rotation;
                   



               // *fx_xform = start_xform;
            }

          

       // println!("update visibility {:?}", is_visible);
        //let bundle =  &mut instance.to_bundle() ;
    }
}




pub fn update_magicfx_anim_frames(
      magic_fx_instance_query: Query<(
        Entity,
        &MagicFxInstanceComponent,
        &Handle<AnimatedMaterial>,
        &Parent
    )  >,

    mut animated_material_assets: ResMut<Assets<AnimatedMaterial>>,
 

    magic_fx_variant_query: Query<&MagicFxVariantComponent>,
    time: Res<Time>,
) {
    let current_time = time.elapsed();

    for (entity, instance_comp, anim_mat_handle, parent) in magic_fx_instance_query.iter() {
       
        let Some(magic_fx_variant) = magic_fx_variant_query.get( parent.get()  ).ok() else {continue};


        let instance = &instance_comp.instance;
        
        let start_time = magic_fx_variant.start_time + instance.start_time_offset;
        let end_time = magic_fx_variant.start_time + instance.end_time_offset;
      

        if let Some(anim_mat) = animated_material_assets.get_mut(anim_mat_handle){


             let total_frames = anim_mat.extension.custom_uniforms.animation_frame_dimension_x * anim_mat.extension.custom_uniforms.animation_frame_dimension_y;
            
              if total_frames <= 1 {continue};

             let total_run_time = instance.end_time_offset - instance.start_time_offset;
             let time_per_frame = total_run_time / total_frames;

             let time_since_start = current_time - start_time;
             let current_frame_index = time_since_start.as_millis() /  time_per_frame.as_millis()   ;

             anim_mat.extension.custom_uniforms.current_animation_frame_index = current_frame_index as u32;

        }
      

       
    }
}




pub fn update_magicfx_tint_color(
      magic_fx_instance_query: Query<(
        Entity,
        &MagicFxInstanceComponent,
        &Handle<AnimatedMaterial>,
        &Parent
    )  >,

    mut animated_material_assets: ResMut<Assets<AnimatedMaterial>>,
 

    magic_fx_variant_query: Query<&MagicFxVariantComponent>,
    time: Res<Time>,
) {
    let current_time = time.elapsed();

    for (entity, instance_comp, anim_mat_handle, parent) in magic_fx_instance_query.iter() {
       
        let Some(magic_fx_variant) = magic_fx_variant_query.get( parent.get()  ).ok() else {continue};


        let instance = &instance_comp.instance;
        
        let start_time = magic_fx_variant.start_time + instance.start_time_offset;
        let end_time = magic_fx_variant.start_time + instance.end_time_offset;
      

         let duration = end_time.as_secs_f32() - start_time.as_secs_f32();
         let elapsed = current_time.as_secs_f32() - start_time.as_secs_f32();

           let lerp_amount: f32 = if duration > 0.0 {
                    elapsed / duration
                } else {
                    0.0
                }
                .clamp(0.0, 1.0);

        if let Some(anim_mat) = animated_material_assets.get_mut(anim_mat_handle){

            if let Some(tint_start_color) = instance.start_tint_color{

            if let Some(tint_end_color) = instance.end_tint_color{

                let lerped_tint_color = lerp_colors(&tint_start_color,&tint_end_color,lerp_amount);
              
                 anim_mat.extension.custom_uniforms.tint_color = lerped_tint_color;

             }
            }
              ;

            
        }
      

       
    }
}



pub fn update_magicfx_billboard_rotation(

    target_query: Query<&GlobalTransform, (With<MagicFxBillboardTarget>, Without<MagicFxBillboardComponent>)>,

    mut magicfx_billboard_query: Query<(&mut Transform, &GlobalTransform), (With<MagicFxBillboardComponent>,Without<MagicFxBillboardTarget>)>

){

    let target_xform = target_query.get_single().cloned().unwrap_or(GlobalTransform::from_xyz(0.0,0.0,0.0));
  
    for( mut magicfx_xform, _magicfx_global_xform) in magicfx_billboard_query.iter_mut(){
 
        // Update the rotation of the billboarded object
        magicfx_xform.look_at ( target_xform.translation() , Vec3::Y  );


    } 


}