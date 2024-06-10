use bevy::asset::load_internal_asset;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use magic_fx_variant::MagicFxVariantManifest;
use shader_variant::ShaderVariantManifest;

use bevy_shader_utils::ShaderUtilsPlugin;

pub mod animated_material;
pub mod euler_transform;
pub mod magic_fx;
pub mod magic_fx_variant;
pub mod shader_variant;
pub(crate) mod util;

pub  mod camera;


pub struct MagicFxPlugin;

// Step 2: Implement the Plugin trait for your struct
impl Plugin for MagicFxPlugin {
    fn build(&self, app: &mut App) {


         load_internal_asset!(
            app,
            MAGIC_FX_SHADER_HANDLE,
            "shaders/magic_fx.wgsl",
            Shader::from_wgsl
        );



        // Step 3: Add systems, resources, and configurations to the Bevy app
        app

            .add_plugins(ShaderUtilsPlugin)
            .add_plugins(RonAssetPlugin::<ShaderVariantManifest>::new(&[
            "shadvar.ron",
            ]))
            .add_plugins(RonAssetPlugin::<MagicFxVariantManifest>::new(&[
                "magicfx.ron",
            ]))
            .add_plugins(MaterialPlugin::<animated_material::AnimatedMaterial > {

                 prepass_enabled: false,
                ..default() 
            })

             .add_systems(Update,( 

                magic_fx::update_magic_fx_variants_added,
                 magic_fx::update_magic_fx_variants,
                 magic_fx::update_magic_fx_instances_visibility,
                  magic_fx::update_magic_fx_instances_translation_scale,
                  magic_fx::update_magicfx_standard_rotation,
                magic_fx::update_magicfx_billboard_rotation,
                  magic_fx::update_magicfx_anim_frames,
                   magic_fx::update_magicfx_tint_color




                ) .chain() ) 

             
            ;

    }
}




          

   pub(crate) const MAGIC_FX_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(7_473_426_912_151_597_127);


