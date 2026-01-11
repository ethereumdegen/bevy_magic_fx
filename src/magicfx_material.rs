use crate::MAGIC_FX_SHADER_HANDLE;
use bevy::prelude::*;

use bevy::shader::ShaderRef; 
use bevy::render::render_resource::*;

use bevy::pbr::{ExtendedMaterial, MaterialExtension };
use bevy::platform::collections::hash_map::HashMap;

 

pub type MagicFxMaterial = ExtendedMaterial<StandardMaterial, MagicFxMaterialBase>;

 
#[derive(Clone, ShaderType, Debug, Reflect )]
pub struct MagicFxMaterialUniforms {
    pub distortion_speed: Vec2, 
    pub scroll_repeats: Vec2, 
    pub scroll_speed: Vec2,
    pub distortion_amount: f32,
    pub distortion_cutoff: f32,
    pub depth_cutoff_offset: f32,
    pub animation_frame_dimension: Vec2, //if this is 1, we know we have a normal static texture.  Otherwise, we have an animated scrolly one 
    pub current_animation_frame_index: u32,

    pub uv_scale_factor: Vec2, 

    pub tint_color: LinearRgba ,
    pub fresnel_power: f32 ,

    pub use_masking_texture: u32,   //bool
    pub animate_masking_texture: u32 ,  //bool

    //pub masking_texture_config_bits: u32
}


impl Default for MagicFxMaterialUniforms {
    fn default() -> Self {
        Self {
             scroll_speed: Vec2::new(0.1,1.0),
             distortion_speed:Vec2::new(3.0,1.0),
             scroll_repeats:Vec2::new(12.0,3.0),

            
            distortion_amount: 0.03,
            distortion_cutoff: 1.0,

            uv_scale_factor: Vec2::new(1.0,1.0),
            
            depth_cutoff_offset: 0.0,
            animation_frame_dimension: Vec2::new(1.0,1.0),
         
            current_animation_frame_index: 0, 
            tint_color: Color::WHITE.into(),
            fresnel_power:  0.0 , //typically like 2.0 if used 

            use_masking_texture: 0,
            animate_masking_texture: 0,
        }
    }
}

// #[derive(Asset, AsBindGroup, TypePath, Debug, Clone, Default)]
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default )]
pub struct MagicFxMaterialBase {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(20)]
    pub custom_uniforms: MagicFxMaterialUniforms,

    #[texture(21)]
    #[sampler(22)]
    pub base_color_texture: Option<Handle<Image>>,

    // Adding masking texture
    #[texture(23)]
    #[sampler(24)]
    pub masking_texture: Option<Handle<Image>>,
  //  #[sampler(24)]
  //  pub masking_sampler: Option<Handle<Sampler>>,
}

impl MagicFxMaterialBase {

    pub fn update_animation(&mut self , time_secs: f32  ) {

        let time_secs_constrained = time_secs % 32000.0 ;

        let anim_frame_dimension =  &self.custom_uniforms. animation_frame_dimension; 


        let factor =  self.custom_uniforms.scroll_speed.x  ; 

        let anim_index_max = anim_frame_dimension.x  as u32   *   anim_frame_dimension.y as u32  ;


        let anim_index =  ( time_secs_constrained * factor ) as u32   % anim_index_max ;

        // println!("{}" , anim_index );
        self.custom_uniforms . current_animation_frame_index = anim_index; 

    }

}

impl MaterialExtension for MagicFxMaterialBase {
    fn fragment_shader() -> ShaderRef {
        MAGIC_FX_SHADER_HANDLE.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        MAGIC_FX_SHADER_HANDLE.into()
    }
}



// ----- 

/*
// Define an enum for the bit positions
#[repr(u32)]
#[derive(Clone,Copy)]
enum MaskingTextureConfigBits {
    UseMaskingTexture = 0,       // Bit 0
    AnimateMaskingTexture = 1,   // Bit 1
    // Add more bits as needed
}

// A helper function to construct the bitfield
fn build_masking_texture_config_bits(flags: &[(MaskingTextureConfigBits, bool)]) -> u32 {
    let mut config_bits = 0;

    for (bit, enabled) in flags {
        if *enabled {
            config_bits |= 1 << *bit as u32;
        }
    }

    config_bits
}*/