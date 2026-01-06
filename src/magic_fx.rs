use crate::rotate_to::{get_rotate_towards,UpDirection};
 
 use bevy::ecs::relationship::Relationship;

use std::ops::Mul;
use std::{ops::Div, time::Duration};

use bevy::prelude::*;


// use bevy:: render:: view::{RenderLayers }; 

use crate::magic_fx_variant::MagicFxStyle;
use crate::{
    magicfx_material::{ MagicFxMaterial, MagicFxMaterialBase  }, 
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


/*
#[derive(Component)]
pub struct MagicFxBillboardComponent {
     
}

#[derive(Component)]
pub struct MagicFxStandardRotationComponent {
     
}

*/



#[derive(Component)]
pub struct MagicFxInstanceComponent {
    pub instance: MagicFxInstance,
  //  pub start_time: Duration, // find from parent 
}



pub(crate) fn magic_fx_comp_plugin(app: &mut App) {
    
    app 
       .add_message::<MagicFxInstanceChildAdded>()
        .add_systems(Update,( 

                 update_magic_fx_variants_added,

                 update_magic_fx_variants,

                 update_magic_fx_instances_visibility,
                  update_magic_fx_instances_translation_scale,
                  update_magicfx_standard_rotation,
                  update_magicfx_billboard_rotation,
                  update_magicfx_billboard_vertical_rotation,
                  update_magicfx_anim_frames,
                  update_magicfx_tint_color




                ) .chain() 
                .in_set( MagixFxStandardUpdateSet )
            ) 

        .add_systems(PostUpdate, update_magic_fx_variants_despawn )
       
       ;

}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MagixFxStandardUpdateSet;




#[derive(Component,Debug,Clone)]
pub struct MagicFxNoAutoTransform; 




#[derive(Message,Debug,Clone)]
pub struct MagicFxInstanceChildAdded {
    pub parent: Entity,
    pub child: Entity 
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

            let anim_mat_bundle =  instance.to_anim_material_bundle();

             let style_component = &instance.fx_style;

            let magic_fx_child = commands
                .spawn(
                  (
                  anim_mat_bundle,   //this includes spatial bundle with mesh 

                    MagicFxInstanceComponent {
                        instance: instance.clone(),
                       // start_time: time.elapsed(),
                    },
                    bevy::light::NotShadowCaster,
                    ChildOf( fx_entity  ) ,
                    style_component.clone() ,

                   //  RenderLayers:: layer(2)   // test 

                  )  
                ) 
                .id();

           // commands.entity(fx_entity).add_child(magic_fx_child);

           

         //   commands.entity(magic_fx_child).insert( style_component.clone() );


           commands.write_message(
                MagicFxInstanceChildAdded {
                    parent: fx_entity.clone(),
                    child: magic_fx_child.clone(), 

                }
            );
          
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

            let Some( max_time_offset ) = magic_fx.max_time_offset else {

               //  magic_fx_comp.start_time = current_time;
                 
                continue

            };
       
            if current_time > magic_fx_comp.start_time + max_time_offset {
                
                 if magic_fx.repeating {
                    //reset 
                     magic_fx_comp.start_time = current_time;
                     
                     }else { 

                    commands.entity(fx_entity).try_insert(DespawnVFX);
                }
            }
        
    }
}


//careful w this 
#[derive(Component)] 
pub struct DespawnVFX;


pub fn update_magic_fx_variants_despawn(

    mut commands :Commands, 

       magic_fx_query: Query<(Entity, & MagicFxVariantComponent), With<DespawnVFX>>

){

    for (magicfx_entity, _variant_comp) in magic_fx_query.iter(){

        if let Some(mut cmd) = commands.get_entity( magicfx_entity  ).ok() {


            cmd.despawn(); 
        }




    } 

}

pub fn update_magic_fx_instances_visibility(
    mut magic_fx_instance_query: Query<(
        Entity,
        &mut Visibility,
        
        &MagicFxInstanceComponent,
        &ChildOf
    )>,

    magic_fx_variant_query: Query<&MagicFxVariantComponent>,
    time: Res<Time>,
) {
    let current_time = time.elapsed();

    for (entity, mut fx_visibility, instance_comp, parent) in magic_fx_instance_query.iter_mut() {
       
        let Some(magic_fx_variant) = magic_fx_variant_query.get( parent.parent()  ).ok() else {continue};


        let instance = &instance_comp.instance;
       // let is_billboarded = &instance.billboard_mesh;
        let start_time = magic_fx_variant.start_time + instance.start_time_offset;
        let end_time = magic_fx_variant.start_time + instance.end_time_offset;

        


        let is_visible = magic_fx_variant.magic_fx.repeating ||   current_time >= start_time && current_time <= end_time;

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
        &ChildOf,
        &MagicFxStyle,
    )>,

    magic_fx_variant_query: Query<&MagicFxVariantComponent>,
    time: Res<Time>,
) {
    let current_time = time.elapsed();

    for (entity,   mut fx_xform, instance_comp, parent, magic_fx_style) in magic_fx_instance_query.iter_mut() {
       
       
        //if magic_fx_style != &MagicFxStyle::Standard &&  magic_fx_style != &MagicFxStyle::Billboard {
        //    continue;
        //}

         let Some(magic_fx_variant) = magic_fx_variant_query.get( parent.parent()  ).ok() else {continue};



        let instance = &instance_comp.instance;
        //let is_billboarded = &instance.billboard_mesh;
        let start_time = magic_fx_variant.start_time + instance.start_time_offset;
        let end_time = magic_fx_variant.start_time + instance.end_time_offset;

        let transform_easing_function_type =  instance.transform_easing_function ;



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
                 let new_transform =  lerp_euler_transforms(&start_xform, &end_xform, lerp_amount, transform_easing_function_type).to_transform();


                 fx_xform.translation = new_transform.translation;
                 fx_xform.scale = new_transform.scale;

                 
            } else {
                // If not visible, or outside the lerp range, you might want to set it to start or end transform
                // Adjust this based on your needs, for now, let's assume it resets to start_xform if not visible
                let new_transform = start_xform.to_transform();

                 fx_xform.translation = new_transform.translation;
                 fx_xform.scale = new_transform.scale;

                   
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
        &ChildOf,
        &MagicFxStyle
    )  >,

    magic_fx_variant_query: Query<&MagicFxVariantComponent>,
    time: Res<Time>,
) {
    let current_time = time.elapsed();

    for (entity,   mut fx_xform, instance_comp, parent, magic_fx_style) in magic_fx_instance_query.iter_mut() {
       
       
        if magic_fx_style != &MagicFxStyle::Standard {
            continue;
        }

         let Some(magic_fx_variant) = magic_fx_variant_query.get( parent.parent()  ).ok() else {continue};


        let instance = &instance_comp.instance;
        
        let start_time = magic_fx_variant.start_time + instance.start_time_offset;
        let end_time = magic_fx_variant.start_time + instance.end_time_offset;

      
        let ease_function_type = instance.transform_easing_function;

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
                 let new_transform =  lerp_euler_transforms(&start_xform, &end_xform, lerp_amount, ease_function_type ).to_transform();

 
                  
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
 
        &MeshMaterial3d< MagicFxMaterial >,
        &ChildOf
    )  >,

    mut animated_material_assets: ResMut<Assets< MagicFxMaterial >>,
 

    magic_fx_variant_query: Query<&MagicFxVariantComponent>,
    time: Res<Time>,
) {
    let current_time = time.elapsed();

    for (entity, instance_comp, anim_mat_handle, parent) in magic_fx_instance_query.iter() {
       
        let Some(magic_fx_variant) = magic_fx_variant_query.get( parent.parent()  ).ok() else {continue};


        let instance = &instance_comp.instance;
        
        let start_time = magic_fx_variant.start_time + instance.start_time_offset;
        let end_time = magic_fx_variant.start_time + instance.end_time_offset;
      

        if let Some(anim_mat) = animated_material_assets.get_mut(anim_mat_handle){


             let total_frames = anim_mat.extension.custom_uniforms.animation_frame_dimension.x as u32 * anim_mat.extension.custom_uniforms.animation_frame_dimension.y as u32;
            
              if total_frames <= 1 {continue};

             let total_run_time = instance.end_time_offset - instance.start_time_offset;
             let time_per_frame = total_run_time / total_frames;

             let time_since_start = current_time - start_time;
             let current_frame_index = ( time_since_start.as_millis() /  time_per_frame.as_millis()  ) % total_frames as u128  ;

             // info!("current frame index {:?}", current_frame_index);

             anim_mat.extension.custom_uniforms.current_animation_frame_index = current_frame_index as u32;

        }
      

       
    }
}




pub fn update_magicfx_tint_color(
      magic_fx_instance_query: Query<(
        Entity,
 
        &MagicFxInstanceComponent,
        &MeshMaterial3d< MagicFxMaterial >,
        &ChildOf
 
    )  >,

    mut animated_material_assets: ResMut<Assets<MagicFxMaterial>>,
 

    magic_fx_variant_query: Query<&MagicFxVariantComponent>,
    time: Res<Time>,
) {
    let current_time = time.elapsed();

    for (entity, instance_comp, anim_mat_handle, parent) in magic_fx_instance_query.iter() {
       
        let Some(magic_fx_variant) = magic_fx_variant_query.get( parent.parent()  ).ok() else {continue};


        let instance = &instance_comp.instance;
        
        let start_time = magic_fx_variant.start_time + instance.start_time_offset;
        let end_time = magic_fx_variant.start_time + instance.end_time_offset;
      
        let color_easing_function_type = instance.color_easing_function; 

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

                let lerped_tint_color = lerp_colors(&tint_start_color,&tint_end_color,lerp_amount , color_easing_function_type);
              
                 anim_mat.extension.custom_uniforms.tint_color = lerped_tint_color.into();

             }
            }
              ;

            
        }
      

       
    }
}




pub fn update_magicfx_billboard_rotation(

    target_query: Query<Entity, With<MagicFxBillboardTarget> >,

    parent_query: Query<&ChildOf>,
    global_xform_query: Query<&GlobalTransform>,

    mut magicfx_billboard_query: Query<(Entity, &mut Transform,  &ChildOf, &MagicFxStyle), Without<MagicFxBillboardTarget> >

){
 

    if let Some(target_entity ) = target_query.single().ok()  {

        
      
        for( billboard_entity, mut magicfx_xform,  _parent, magic_fx_style) in magicfx_billboard_query.iter_mut(){
                
           if magic_fx_style != &MagicFxStyle::Billboard  {
                continue;
            }
          

 

             let new_rotation = get_rotate_towards (
                billboard_entity,

                target_entity,
                 UpDirection::Target,   // use the cameras vertical as billboard vertical 
                 &parent_query,
                 &global_xform_query,


                );

             if let Some(new_rotation) = new_rotation {
                magicfx_xform.rotation =  new_rotation ; 

             }
            
        }       



    }


}




pub fn update_magicfx_billboard_vertical_rotation(

    target_query: Query<Entity, With<MagicFxBillboardTarget> >,

    parent_query: Query<&ChildOf>,
    global_xform_query: Query<&GlobalTransform>,

    mut magicfx_billboard_query: Query<(Entity, &mut Transform,  &ChildOf, &MagicFxStyle), Without<MagicFxBillboardTarget> >

){
 

    if let Some(target_entity ) = target_query.single().ok()  {

    //let Some(target_xform) = global_xform_query.get(target_entity).ok() else {return};
    

       
      
        for( billboard_entity, mut magicfx_xform,  _parent, magic_fx_style) in magicfx_billboard_query.iter_mut(){
                
           if magic_fx_style != &MagicFxStyle::BillboardVertically  {
                continue;
            }
          

 

             let new_rotation = get_rotate_towards (
                billboard_entity,

                target_entity,
                 UpDirection::Parent,   // use the parents vertical as billboarding vertical 
                 &parent_query,
                 &global_xform_query,


                );

             if let Some(new_rotation) = new_rotation {
                magicfx_xform.rotation =  new_rotation ; 

             }
            
        }       



    }


}


/* 
pub fn update_magicfx_billboard_vertical_rotation(

    target_query: Query<Entity, With<MagicFxBillboardTarget> >,

    global_xform_query: Query<&GlobalTransform>,

    mut magicfx_billboard_query: Query<(Entity, &mut Transform,  &Parent, &MagicFxStyle), Without<MagicFxBillboardTarget> >

){

    //let target_xform = target_query.get_single().cloned() ;

    if let Some(target_entity ) = target_query.get_single().ok()  {

        let Some(target_xform) = global_xform_query.get(target_entity).ok() else {return};
    

       
      
        for( billboard_entity, mut magicfx_xform,  parent, magic_fx_style) in magicfx_billboard_query.iter_mut(){
            

             if magic_fx_style != &MagicFxStyle::BillboardVertically {
                continue;
            }
          

            let Some(magicfx_global_xform) = global_xform_query.get(billboard_entity).ok() else {continue};

            let parent_entity = parent.get();

             let Some(parent_global_xform) = global_xform_query.get(parent_entity).ok() else {continue};



         
            let dir = (target_xform.translation() - magicfx_global_xform.translation()).normalize();

            // Calculate yaw and pitch
            let yaw = -dir.z.atan2(dir.x);
            let pitch = -dir.y.asin();


            let (_, parent_rotation , _ ) = parent_global_xform.to_scale_rotation_translation();
            let corrective_rotation = parent_rotation.inverse();

        //    magicfx_xform.rotation = magicfx_xform.rotation * corrective_rotation;// now its global rot should be identity 

            // Create quaternion from yaw and pitch
            let yaw_rotation = Quat::from_rotation_y(yaw);
         //   let pitch_rotation = Quat::from_rotation_x(pitch);

            // Combine rotations
            magicfx_xform.rotation = yaw_rotation * corrective_rotation ; // * pitch_rotation;


        }       



    }


}
*/