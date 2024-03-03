use std::time::Duration;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::animated_material::{self, AnimatedMaterial, AnimatedMaterialBundle, AnimatedMaterialExtension};


use crate::euler_transform::EulerTransform;
use crate::shader_variant::{ShaderVariant, ShaderVariantManifest};


//use bevy::utils::HashMap;

//this gets loaded in

#[derive(Debug, Clone, Asset, Serialize, Deserialize)]
pub struct MagicFxVariantManifest { 

    pub name: String,  

    pub magic_fx_instances: Vec<MagicFxInstanceManifest>,

    pub max_time: f32


} 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicFxInstanceManifest {

	pub shader_variant_name: String,
	pub mesh_name:String, 
	pub start_time_offset: f32,
	pub end_time_offset:f32,
	pub start_transform: EulerTransform,
	pub end_transform: EulerTransform 

}

impl TypePath for MagicFxVariantManifest {
    fn short_type_path() -> &'static str {
        "mfxvar.ron"
    }
    fn type_path() -> &'static str {
        "mfxvar.ron"
    }
}


#[derive(Debug, Clone)]
pub struct MagicFxVariant  { 

    pub name: String,  

    pub magic_fx_instance: Vec<MagicFxInstance>,

    pub current_time: Duration,
    pub max_time: Duration


} 

impl MagicFxVariant {

	pub fn from_manifest(  
      		manifest: &MagicFxVariantManifest ,
		   asset_server: &Res< AssetServer>
		 ) -> Self {


		Self {
			name: manifest.name.clone(),
			current_time: Duration::from_secs_f32(0.0),
			max_time: Duration::from_secs_f32( manifest.max_time) ,
			magic_fx_instance:  manifest.magic_fx_instances.clone().drain(..).map( |instance_manifest|
				MagicFxInstance::from_manifest( instance_manifest, asset_server)
				).collect() 

		}


	}

}

#[derive(Debug, Clone)]
pub struct MagicFxInstance  {

	pub shader_variant: Handle<AnimatedMaterialExtension>,
	pub mesh: Handle<Mesh>, 
	pub start_time_offset: Duration,
	pub end_time_offset:Duration,
	pub start_transform: Transform,
	pub end_transform: Transform 

}



impl MagicFxInstance {

	pub fn from_manifest( 
		manifest: MagicFxInstanceManifest,
		asset_server: &Res< AssetServer>,
   
		//shader_variant_assets: Res<Assets<ShaderVariant>>,


		// mut custom_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, custom_material::ScrollingMaterial>>>,
 



	 ) -> Self {

		 
		Self {


			end_time_offset: Duration::from_secs_f32(  manifest.end_time_offset ),
			shader_variant: asset_server.load( & manifest.shader_variant_name ) ,

		    mesh:asset_server.load( & manifest.mesh_name ),
		    start_time_offset: Duration::from_secs_f32( manifest.start_time_offset ),
		    start_transform: manifest.start_transform.to_transform(),
		    end_transform:  manifest.end_transform.to_transform(),




		}  
	}



	pub fn to_bundle(&self) -> AnimatedMaterialBundle {

		let initial_transform = self.start_transform;

		    animated_material::AnimatedMaterialBundle {
                            mesh:  self.mesh.clone(),
                            material:  self.shader_variant.clone(),
                          
                            transform: self.start_transform,
                        
                            ..default()
                        } 

	}
}

/*

need to loop through these animated mateiral bundles and change the xform over time 
*/