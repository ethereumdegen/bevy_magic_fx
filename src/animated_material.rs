use crate::MAGIC_FX_SHADER_HANDLE;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;

use bevy::pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod};
use bevy::utils::HashMap;

use crate::shader_variant::ShaderVariantManifest;


pub type AnimatedMaterial = ExtendedMaterial<StandardMaterial, AnimatedMaterialBase>;

 

pub fn build_animated_material(
    shader_variant_manifest: &ShaderVariantManifest,
    texture_handles_map: & HashMap<String,Handle<Image>>, 

    ) -> Result<AnimatedMaterial,String>{


   let base_color = (&shader_variant_manifest.color).clone();
   let emissive = (&shader_variant_manifest.emissive).clone();

  

   let texture_handle = texture_handles_map.get(&shader_variant_manifest.texture).ok_or(  format!("missing texture {:?}", &shader_variant_manifest.texture )  )?;

   let masking_texture_handle = shader_variant_manifest.masking_texture.as_ref().map(|m|  texture_handles_map.get( m ) ) .flatten();
    

   let use_masking_texture = match masking_texture_handle {
    Some(_) => 1,
    None => 0
   };

   Ok(
    ExtendedMaterial {
                    base: StandardMaterial {
                        base_color : base_color.into(),
                        emissive: emissive.into(),
                        // can be used in forward or deferred mode.
                        opaque_render_method: OpaqueRendererMethod::Auto,
                        alpha_mode: AlphaMode::Blend,

                        double_sided: true,
                        cull_mode: None,
                        //unlit:true, 



                        ..Default::default()
                    },
                    extension: AnimatedMaterialBase{
                        base_color_texture: Some(texture_handle.clone()),

                        masking_texture: masking_texture_handle.cloned(),

                       
                        custom_uniforms: AnimatedMaterialUniforms {
                            scroll_speed: shader_variant_manifest.animation_speed,
                            distortion_speed: shader_variant_manifest.distortion_speed,

                            uv_scale_factor: shader_variant_manifest.uv_scale_factor.unwrap_or(Vec2::new(1.0,1.0)),
                         
                            distortion_amount: shader_variant_manifest.distortion_amount,
                            distortion_cutoff: 1.0,
                            scroll_repeats: shader_variant_manifest.scroll_repeats,
                            
                            fresnel_power: shader_variant_manifest.fresnel_power.unwrap_or( 0.0 ), 
                            depth_cutoff_offset: shader_variant_manifest.depth_cutoff_offset.unwrap_or( 0.0 ),  // typically use  0.05, like for magic fire that is rendered behind stuff 
                            animation_frame_dimension: shader_variant_manifest.animation_frame_dimensions.map(|d|  Vec2::new(d[0] as f32,d[1] as f32)  ).unwrap_or(  Vec2::new(1.0,1.0) ),


                            use_masking_texture ,
                         //   animation_frame_dimension_y: shader_variant_manifest.animation_frame_dimensions.map(|d| d[1]).unwrap_or(   1 ), 

                            ..default()
                        },
                        ..default()
                    },
                }

      )
}

//pub type AnimatedMaterialExtension = ExtendedMaterial<StandardMaterial, AnimatedMaterial>;
//pub type AnimatedMaterialBundle = MaterialMeshBundle<AnimatedMaterial >;

#[derive(Clone, ShaderType, Debug)]
pub struct AnimatedMaterialUniforms {
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
    pub use_masking_texture: u32
}
impl Default for AnimatedMaterialUniforms {
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

            use_masking_texture: 0
        }
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone, Default)]
pub struct AnimatedMaterialBase {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(20)]
    pub custom_uniforms: AnimatedMaterialUniforms,

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

impl MaterialExtension for AnimatedMaterialBase {
    fn fragment_shader() -> ShaderRef {
        MAGIC_FX_SHADER_HANDLE.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        MAGIC_FX_SHADER_HANDLE.into()
    }
}
