use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};



//this gets loaded in

//this ends up as pub shader_variant_materials: HashMap<String, Handle<AnimatedMaterial>>,

//pub type ShaderVariant = AnimatedMaterial;

#[derive(Debug, Clone, Asset, Serialize, Deserialize)]
pub struct ShaderVariantManifest {
   
    pub texture: String,

    pub animation_speed: Vec2,
    pub distortion_speed: Vec2,

    pub scroll_repeats: Vec2,

    pub distortion_amount: f32,

    pub color: Color,
    pub emissive: Color,
 
    pub depth_cutoff_offset: Option<f32> ,   // use depth bias for this ? 

    pub fresnel_power: Option<f32>,

    pub animation_frame_dimensions: Option<[u32;2]>
}

impl TypePath for ShaderVariantManifest {
    fn short_type_path() -> &'static str {
        "shadvar.ron"
    }
    fn type_path() -> &'static str {
        "shadvar.ron"
    }
}
 