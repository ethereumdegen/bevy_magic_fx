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

    ) -> Option<AnimatedMaterial>{


   let base_color = (&shader_variant_manifest.color).clone();
   let emissive = (&shader_variant_manifest.emissive).clone();

   let texture_handle = texture_handles_map.get(&shader_variant_manifest.texture)?;
 
   Some(
    ExtendedMaterial {
                    base: StandardMaterial {
                        base_color,
                        emissive,
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

                       
                        custom_uniforms: AnimatedMaterialUniforms {
                            scroll_speed: shader_variant_manifest.animation_speed,
                            distortion_speed: shader_variant_manifest.distortion_speed,
                         
                            distortion_amount: shader_variant_manifest.distortion_amount,
                            distortion_cutoff: 1.0,
                            scroll_repeats: shader_variant_manifest.scroll_repeats,
                            
                            depth_cutoff_offset: shader_variant_manifest.depth_cutoff_offset.unwrap_or( 0.0 ),  // typically use  0.05, like for magic fire that is rendered behind stuff 
                            animation_frame_dimension: shader_variant_manifest.animation_frame_dimensions.map(|d|  Vec2::new(d[0] as f32,d[1] as f32)  ).unwrap_or(  Vec2::new(1.0,1.0) ), 
                         //   animation_frame_dimension_y: shader_variant_manifest.animation_frame_dimensions.map(|d| d[1]).unwrap_or(   1 ), 

                            ..default()
                        },
                        ..default()
                    },
                }

      )
}

//pub type AnimatedMaterialExtension = ExtendedMaterial<StandardMaterial, AnimatedMaterial>;
pub type AnimatedMaterialBundle = MaterialMeshBundle<AnimatedMaterial >;

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

    pub tint_color: Color 
}
impl Default for AnimatedMaterialUniforms {
    fn default() -> Self {
        Self {
             scroll_speed: Vec2::new(0.1,1.0),
             distortion_speed:Vec2::new(3.0,1.0),
             scroll_repeats:Vec2::new(12.0,3.0),

            
            distortion_amount: 0.03,
            distortion_cutoff: 1.0,
            
            depth_cutoff_offset: 0.0,
            animation_frame_dimension: Vec2::new(1.0,1.0),
         
            current_animation_frame_index: 0, 
            tint_color: Color::WHITE
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
}

impl MaterialExtension for AnimatedMaterialBase {
    fn fragment_shader() -> ShaderRef {
        "shaders/magic_fx.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/magic_fx.wgsl".into()
    }
}
