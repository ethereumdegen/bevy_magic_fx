use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::animated_material::AnimatedMaterial;

 

//this gets loaded in

//this ends up as pub shader_variant_materials: HashMap<String, Handle<AnimatedMaterial>>,

pub type ShaderVariant = AnimatedMaterial;

#[derive(Debug, Clone, Asset, Serialize, Deserialize)]
pub struct ShaderVariantManifest {
    //     variant: HashMap<String, ShaderVariant >
    pub name: String, //used to load it
    pub texture: String,
    pub animation_speed: f32,
    pub color: Color,
    pub emissive: Vec3, 

}
 

impl TypePath for ShaderVariantManifest {
    fn short_type_path() -> &'static str {
        "shadvar.ron"
    }
    fn type_path() -> &'static str {
        "shadvar.ron"
    }
}