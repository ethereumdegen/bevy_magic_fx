use bevy_materialize::prelude::GenericMaterial;
use bevy_materialize::generic_material::GenericMaterial3d;
use std::time::Duration;

use bevy::pbr::{ExtendedMaterial, OpaqueRendererMethod};
use bevy::prelude::*;
 
use bevy::platform::collections::hash_map::HashMap;
 
 
 
use serde::{Deserialize, Serialize};
 

use crate::euler_transform::EulerTransform;
 
//use crate::shader_variant::ShaderVariantManifest;
 

#[derive(Debug, Clone, Asset, Serialize, Deserialize)]
pub struct MagicFxVariantManifest {
  //  pub name: String,

    pub magic_fx_instances: Vec<MagicFxInstanceManifest>,

    pub max_time: Option<f32>,

    pub repeating: bool 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicFxInstanceManifest {
    pub shader_variant_name: String,

    pub mesh_name: String,
    //pub billboard_mesh : bool, 
    pub fx_style: MagicFxStyle,

    pub start_time_offset: f32,
    pub end_time_offset: f32,


    pub start_transform: EulerTransform,
    pub end_transform: EulerTransform,

    #[serde(default = "default_easing_function")]
    pub transform_easing_function: EaseFunction ,

     pub start_tint_color: Option<Color>,
     pub end_tint_color: Option<Color>,

      #[serde(default = "default_easing_function")]
    pub color_easing_function: EaseFunction ,

}

impl TypePath for MagicFxVariantManifest {
    fn short_type_path() -> &'static str {
        "magicfx.ron"
    }
    fn type_path() -> &'static str {
        "magicfx.ron"
    }
}


fn default_easing_function() -> EaseFunction {

    EaseFunction::Linear
}

#[derive(Clone,Serialize,Deserialize,Debug,Component,Eq,PartialEq,Hash )]
pub enum MagicFxStyle {
    Standard, 
    Billboard , 
    BillboardVertically, //only spin about Z axis  
    Beam 

}

#[derive(Debug, Clone)]
pub struct MagicFxVariant {
  //  pub name: String,

    pub magic_fx_instances: Vec<MagicFxInstance>,

    // pub current_time: Duration,
    pub max_time_offset: Option<Duration>,

    pub repeating: bool 
}

impl MagicFxVariant {
    pub fn from_manifest(
        manifest: &MagicFxVariantManifest,
        
        mesh_handles_map: &HashMap<String, Handle<Mesh>>,
     
       // animated_materials_map: &HashMap<String,Handle<MagicFxMaterial>>,

       //  animated_materials_assets: &Res<Assets<MagicFxMaterial>>,


         generic_materials_map: &HashMap<String,Handle<GenericMaterial>>,

         generic_materials_assets: &Res<Assets< GenericMaterial >>,
         
        asset_server: &ResMut<AssetServer>
     
    ) -> Result<Self> {
     
      let magic_fx_instances: Vec<MagicFxInstance> = manifest
            .magic_fx_instances
            .clone()
            .into_iter()
            .map(|instance_manifest| {
                MagicFxInstance::from_manifest(
                    instance_manifest,
                    mesh_handles_map,
                    generic_materials_map,
                    generic_materials_assets,
                    asset_server,
                )
            })
            .collect::<Result<Vec<MagicFxInstance>, _>>()?; // Collect Results and propagate errors

        Ok(Self {
            repeating: manifest.repeating,
            max_time_offset: manifest.max_time.map(|t| Duration::from_secs_f32(t)),
            magic_fx_instances,
        })
        
        
    }

    
}

#[derive(Debug, Clone)]
pub struct MagicFxInstance {
    
    pub mesh_handle: Handle<Mesh>,
    pub fx_style : MagicFxStyle, 
    pub start_time_offset: Duration,
    pub end_time_offset: Duration,
    pub start_transform: EulerTransform,
    pub end_transform: EulerTransform,
    pub transform_easing_function: EaseFunction ,

     pub shader_material_handle: Handle< GenericMaterial >,

     pub start_tint_color: Option<Color>,
     pub end_tint_color: Option<Color>,
     pub color_easing_function: EaseFunction, 
    

}

impl MagicFxInstance {
  pub fn from_manifest(
        manifest: MagicFxInstanceManifest,
        mesh_handles_map: &HashMap<String, Handle<Mesh>>,
        generic_materials_map: &HashMap<String, Handle<GenericMaterial>>,
        generic_materials_assets: &Res<Assets<GenericMaterial>>,
        asset_server: &ResMut<AssetServer>
    ) -> Result<Self> {
        let mesh_handle = mesh_handles_map
            .get(&manifest.mesh_name)
            .ok_or_else(|| format!("Mesh '{}' not found in mesh_handles_map", manifest.mesh_name))?;

        println!("loading shader material handle {:?}", &manifest.shader_variant_name);
        
        let shader_material_handle = generic_materials_map
            .get(&manifest.shader_variant_name)
            .ok_or_else(|| format!("Shader material '{}' not found in generic_materials_map", manifest.shader_variant_name))?
            .clone();

        Ok(Self { 
            end_time_offset: Duration::from_secs_f32(manifest.end_time_offset),
            shader_material_handle,
            mesh_handle: mesh_handle.clone(),
            fx_style: manifest.fx_style.clone(),
            start_time_offset: Duration::from_secs_f32(manifest.start_time_offset),
            start_transform: manifest.start_transform,
            end_transform: manifest.end_transform,
            transform_easing_function: manifest.transform_easing_function, 
            start_tint_color: manifest.start_tint_color,
            end_tint_color: manifest.end_tint_color,
            color_easing_function: manifest.color_easing_function,
        })
    }

   
    pub fn to_anim_material_bundle(&self) ->  impl Bundle {
        let shader_material = &self.shader_material_handle;

        return (

            Mesh3d( self.mesh_handle.clone() ) ,
            GenericMaterial3d( shader_material.clone() ),
            self.start_transform.clone().to_transform(),
             Visibility::Hidden,
         )

       
        
    }
}

/*

need to loop through these animated mateiral bundles and change the xform over time
*/
