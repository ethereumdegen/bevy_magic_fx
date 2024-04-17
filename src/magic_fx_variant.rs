use std::time::Duration;

use bevy::pbr::{ExtendedMaterial, OpaqueRendererMethod};
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

use crate::animated_material::{
    self, AnimatedMaterialBase, AnimatedMaterialBundle, AnimatedMaterial ,
};

use crate::euler_transform::EulerTransform;
use crate::shader_variant::ShaderVariantManifest;
 

#[derive(Debug, Clone, Asset, Serialize, Deserialize)]
pub struct MagicFxVariantManifest {
  //  pub name: String,

    pub magic_fx_instances: Vec<MagicFxInstanceManifest>,

    pub max_time: f32,

    pub repeating: bool 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicFxInstanceManifest {
    pub shader_variant_name: String,

    pub mesh_name: String,
    pub billboard_mesh : bool, 

    pub start_time_offset: f32,
    pub end_time_offset: f32,
    pub start_transform: EulerTransform,
    pub end_transform: EulerTransform,

     pub start_tint_color: Option<Color>,
     pub end_tint_color: Option<Color>,

}

impl TypePath for MagicFxVariantManifest {
    fn short_type_path() -> &'static str {
        "magicfx.ron"
    }
    fn type_path() -> &'static str {
        "magicfx.ron"
    }
}

#[derive(Debug, Clone)]
pub struct MagicFxVariant {
  //  pub name: String,

    pub magic_fx_instances: Vec<MagicFxInstance>,

    // pub current_time: Duration,
    pub max_time_offset: Duration,

    pub repeating: bool 
}

impl MagicFxVariant {
    pub fn from_manifest(
        manifest: &MagicFxVariantManifest,
        
        mesh_handles_map: &HashMap<String, Handle<Mesh>>,
     
        animated_materials_map: &HashMap<String,Handle<AnimatedMaterial>>,
     
    ) -> Option<Self> {
       // let current_time = time.elapsed();

         let magic_fx_instances: Vec<MagicFxInstance> = manifest
            .magic_fx_instances
            .clone()
            .into_iter() // Use into_iter instead of drain(..) to consume the vector
            .map(|instance_manifest| {
                MagicFxInstance::from_manifest(
                    instance_manifest,
                    mesh_handles_map,
                    animated_materials_map,
                )
            })
            .collect::<Option<Vec<MagicFxInstance>>>()?; // Early return None if any item is None

        Some(Self {
          //  name: manifest.name.clone(),
            repeating: manifest.repeating,
            max_time_offset: Duration::from_secs_f32(manifest.max_time),
            magic_fx_instances,
        })
        
    }

    
}

#[derive(Debug, Clone)]
pub struct MagicFxInstance {
    
    pub mesh_handle: Handle<Mesh>,
    pub billboard_mesh : bool, 
    pub start_time_offset: Duration,
    pub end_time_offset: Duration,
    pub start_transform: EulerTransform,
    pub end_transform: EulerTransform,
     pub shader_material_handle: Handle<AnimatedMaterial>,

     pub start_tint_color: Option<Color>,
     pub end_tint_color: Option<Color>,



}

impl MagicFxInstance {
    pub fn from_manifest(
        manifest: MagicFxInstanceManifest,
 
        mesh_handles_map: &HashMap<String, Handle<Mesh>>,
 
        animated_materials_map: &HashMap<String,Handle<AnimatedMaterial>>,
    ) -> Option<Self> {

        let mesh_handle = mesh_handles_map.get(&manifest.mesh_name)?;
 

         let shader_material_handle  = animated_materials_map
         .get(&manifest.shader_variant_name)?.clone();
       
        Some(Self { 

            end_time_offset: Duration::from_secs_f32(manifest.end_time_offset),
         
         	shader_material_handle,

        
            mesh_handle: mesh_handle.clone (),
            billboard_mesh: manifest.billboard_mesh,
            start_time_offset: Duration::from_secs_f32(manifest.start_time_offset),
            start_transform: manifest.start_transform ,
            end_transform: manifest.end_transform ,

            start_tint_color: manifest.start_tint_color,
            end_tint_color: manifest.end_tint_color,
        })
    }

   
    pub fn to_bundle(&self) -> AnimatedMaterialBundle {
        let shader_material = &self.shader_material_handle;

        return  animated_material::AnimatedMaterialBundle {
                mesh: self.mesh_handle.clone(),
                material: shader_material.clone(),

                transform: self.start_transform.clone().to_transform(),
                visibility: Visibility::Hidden,

                ..default()
            }
        
    }
}

/*

need to loop through these animated mateiral bundles and change the xform over time
*/
