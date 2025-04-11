use bevy_materialize::MaterializePlugin;
use bevy_materialize::prelude::*;
use bevy::asset::load_internal_asset;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use magic_fx_variant::MagicFxVariantManifest;
//use shader_variant::ShaderVariantManifest;



//use bevy_shader_utils::ShaderUtilsPlugin;

pub mod magicfx_material;
pub mod euler_transform;
pub mod magic_fx;
pub mod magic_fx_variant;
// pub mod shader_variant;  //use materialize now 
pub mod magic_fx_beam;
pub(crate) mod util;

pub mod camera;

pub mod rotate_to;

//pub mod interpolation;

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

           .add_plugins( MaterializePlugin::new(TomlMaterialDeserializer) )

            //.add_plugins(ShaderUtilsPlugin)
           /* .add_plugins(RonAssetPlugin::<ShaderVariantManifest>::new(&[
            "shadvar.ron",
            ])) */
            .add_plugins(RonAssetPlugin::<MagicFxVariantManifest>::new(&[
                "magicfx.ron",
            ]))
            .add_plugins(MaterialPlugin::<magicfx_material::MagicFxMaterial > {

                 prepass_enabled: false,
                ..default() 
            })

            //do both registrations! 
             .register_generic_material::< magicfx_material::MagicFxMaterial >()
             .register_generic_material_shorthand::< magicfx_material::MagicFxMaterial >("MagicFxMaterial")

            

            .add_plugins(magic_fx_beam::magic_fx_beam_plugin)
            .add_plugins(magic_fx::magic_fx_comp_plugin)
 
            ;

    }
}




          

   pub(crate) const MAGIC_FX_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(7_473_426_912_151_597_127);


