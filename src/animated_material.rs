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
                            scroll_speed_x: shader_variant_manifest.animation_speed.x,
                            scroll_speed_y: shader_variant_manifest.animation_speed.y,
                            distortion_speed_x: shader_variant_manifest.distortion_speed.x,
                            distortion_speed_y: shader_variant_manifest.distortion_speed.y,
                            distortion_amount: shader_variant_manifest.distortion_amount,
                            distortion_cutoff: 1.0,
                            scroll_repeats_x: shader_variant_manifest.scroll_repeats.x,
                            scroll_repeats_y: shader_variant_manifest.scroll_repeats.y,
                            depth_cutoff_offset: shader_variant_manifest.depth_cutoff_offset.unwrap_or( 0.0 ),  // typically use  0.05, like for magic fire that is rendered behind stuff 
                            animation_frame_dimension_x: shader_variant_manifest.animation_frame_dimensions.map(|d| d[0]).unwrap_or(   1 ), 
                             animation_frame_dimension_y: shader_variant_manifest.animation_frame_dimensions.map(|d| d[1]).unwrap_or(   1 ), 

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
    pub distortion_speed_x: f32,
    pub distortion_speed_y: f32,
    pub scroll_repeats_x: f32,
    pub scroll_repeats_y: f32,
    pub scroll_speed_x: f32,
    pub scroll_speed_y: f32,
    pub distortion_amount: f32,
    pub distortion_cutoff: f32,
    pub depth_cutoff_offset: f32,
    pub animation_frame_dimension_x: u32, //if this is 1, we know we have a normal static texture.  Otherwise, we have an animated scrolly one 
    pub animation_frame_dimension_y: u32,
    pub current_animation_frame_index: u32,

    pub tint_color: Color 
}
impl Default for AnimatedMaterialUniforms {
    fn default() -> Self {
        Self {
            scroll_speed_x: 0.1,
            scroll_speed_y: 1.0,
            distortion_speed_x: 3.0,
            distortion_speed_y: 1.0,
            distortion_amount: 0.03,
            distortion_cutoff: 1.0,
            scroll_repeats_x: 12.0,
            scroll_repeats_y: 3.0,
            depth_cutoff_offset: 0.0,
            animation_frame_dimension_x : 1,
            animation_frame_dimension_y: 1,
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
