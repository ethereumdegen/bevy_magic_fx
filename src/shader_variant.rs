use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use crate::animated_material::AnimatedMaterial;

//this gets loaded in

//this ends up as pub shader_variant_materials: HashMap<String, Handle<AnimatedMaterial>>,

//pub type ShaderVariant = AnimatedMaterial;

#[derive(Debug, Clone, Asset, Serialize, Deserialize)]
pub struct ShaderVariantManifest {
    //     variant: HashMap<String, ShaderVariant >
    pub name: String, //used to load it
    pub texture: String,

    pub animation_speed: Vec2,
    pub distortion_speed: Vec2,

    pub scroll_repeats: Vec2,

    pub distortion_amount: f32,

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

/*
#[derive(Debug, Clone )]
pub struct ShaderVariant  {
    //     variant: HashMap<String, ShaderVariant >
    pub name: String, //used to load it
    pub texture: Handle<Image>,
    pub animation_speed: f32,
    pub color: Color,
    pub emissive: Vec3,

}

impl ShaderVariant {

    pub fn from_manifest(
        manifest: &ShaderVariantManifest,
        texture_handles_map: &HashMap<String, Handle<Image>>

        ) -> Option<Self>{



        let texture_handle = texture_handles_map.get( &manifest.texture ).unwrap().clone_weak();


        Some( Self {



            name: manifest.name.clone(),
            texture: texture_handle,
            color: manifest.color.clone(),
            animation_speed: manifest.animation_speed.clone(),
            emissive: manifest.emissive.clone()


        })


    }
}

*/
