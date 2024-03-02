use std::time::Duration;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::animated_material::AnimatedMaterial;


use crate::euler_transform::EulerTransform;
use crate::shader_variant::ShaderVariantManifest;


//use bevy::utils::HashMap;

//this gets loaded in

#[derive(Debug, Clone, Asset, Serialize, Deserialize)]
pub struct MagicFxVariantManifest { 

    pub name: String,  

    pub magic_fx_instance: Vec<MagicFxInstanceManifest>,


} 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicFxInstanceManifest {

	pub shader_variant_name: String,
	pub mesh_name:String, 
	pub start_time_offset: Duration,
	pub end_time_offset:Duration,
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


} 

#[derive(Debug, Clone)]
pub struct MagicFxInstance  {

	pub shader_variant: Handle<AnimatedMaterial>,
	pub mesh:Handle<Mesh>, 
	pub start_time_offset: Duration,
	pub end_time_offset:Duration,
	pub start_transform: Transform,
	pub end_transform: Transform 

}