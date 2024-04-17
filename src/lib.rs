use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use magic_fx_variant::MagicFxVariantManifest;
use shader_variant::ShaderVariantManifest;

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
        // Step 3: Add systems, resources, and configurations to the Bevy app
        app
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
             .add_systems(Update, magic_fx::update_magic_fx_variants_added)
            .add_systems(Update, magic_fx::update_magic_fx_variants)
            .add_systems(Update, magic_fx::update_magic_fx_instances_visibility)
            .add_systems(Update, magic_fx::update_magic_fx_instances_translation_scale )
             .add_systems(Update, magic_fx::update_magicfx_standard_rotation)
            .add_systems(Update, magic_fx::update_magicfx_billboard_rotation)
            .add_systems(Update, magic_fx::update_magicfx_anim_frames)
            .add_systems(Update, magic_fx::update_magicfx_tint_color)
            ;

    }
}

