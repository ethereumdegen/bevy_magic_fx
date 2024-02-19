use bevy::asset::VisitAssetDependencies;
use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::render_resource::*;

use bevy::render::render_asset::RenderAssets;

use bevy::pbr::StandardMaterialUniform;
use bevy::pbr::StandardMaterialFlags;


/*


This is where we set up all of our pipeline bindings

reference:
https://github.com/bevyengine/bevy/blob/main/assets/shaders/custom_material.wgsl



*/


#[derive(Clone, ShaderType,Default )]
pub struct Repeats {
    pub horizontal: u32,
    pub vertical: u32,
}


#[derive(Clone, ShaderType )]
pub struct CustomMaterialUniforms {
    pub distortion_speed_x: f32,
    pub distortion_speed_y: f32,
    pub scroll_repeats_x:  f32 ,
    pub scroll_repeats_y:  f32 ,
    pub scroll_speed_x: f32,    
    pub scroll_speed_y: f32,    
    pub distortion_amount: f32 ,
    pub distortion_cutoff: f32  
    
}
impl Default for CustomMaterialUniforms{

    fn default() -> Self{

        Self{
            scroll_speed_x : 0.1,
            scroll_speed_y : 1.0,
            distortion_speed_x: 3.0,
            distortion_speed_y: 1.0,
            distortion_amount: 0.03,
            distortion_cutoff: 1.0,
            scroll_repeats_x: 12.0,
            scroll_repeats_y: 3.0,
        }  
    } 
}
 
#[derive(AsBindGroup, TypeUuid, TypePath, Clone, Default)]
#[uuid = "4acc53dd-2cfd-48ba-b659-c0e1a9bc0bdb"]    
#[uniform(0, StandardMaterialUniform)]
pub struct ScrollingMaterial {
  
    
    #[texture(1)]
    #[sampler(2)]    //wtf ? 
    pub base_color_texture_1: Option<Handle<Image>>,

    #[texture(3)]
    #[sampler(4)]   
    pub emissive_texture: Option<Handle<Image>>,

    #[texture(5)]
    #[sampler(6)] 
    pub metallic_roughness_texture: Option<Handle<Image>>,

    #[texture(7)]
    #[sampler(8)] 
    pub occlusion_texture: Option<Handle<Image>>,

   
    #[uniform(20)]
    pub custom_uniforms: CustomMaterialUniforms,

    #[texture(21)]
    #[sampler(22)]
    pub base_color_texture: Option<Handle<Image>>,

     
  
    pub base_color: Color

 
}

impl Material for ScrollingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/scrolling.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Multiply   //IMPORTANT 
    }
}

impl Asset for ScrollingMaterial {}

impl VisitAssetDependencies for ScrollingMaterial {
    fn visit_dependencies(&self, visit: &mut impl FnMut(bevy::asset::UntypedAssetId)) {
        //what to do here ?
    }
}


 
 

impl AsBindGroupShaderType<StandardMaterialUniform> for ScrollingMaterial {
    fn as_bind_group_shader_type(&self, images: &RenderAssets<Image>) -> StandardMaterialUniform {
        let mut flags = StandardMaterialFlags::NONE;
        /*if self.base_color_texture.is_some() {
            flags |= StandardMaterialFlags::BASE_COLOR_TEXTURE;
        }
        if self.emissive_texture.is_some() {
            flags |= StandardMaterialFlags::EMISSIVE_TEXTURE;
        }
        if self.metallic_roughness_texture.is_some() {
            flags |= StandardMaterialFlags::METALLIC_ROUGHNESS_TEXTURE;
        }
        if self.occlusion_texture.is_some() {
            flags |= StandardMaterialFlags::OCCLUSION_TEXTURE;
        }
        if self.double_sided {
            flags |= StandardMaterialFlags::DOUBLE_SIDED;
        }
        if self.unlit {
            flags |= StandardMaterialFlags::UNLIT;
        }
        if self.fog_enabled {
            flags |= StandardMaterialFlags::FOG_ENABLED;
        }
        if self.depth_map.is_some() {
            flags |= StandardMaterialFlags::DEPTH_MAP;
        }
        #[cfg(feature = "pbr_transmission_textures")]
        {
            if self.specular_transmission_texture.is_some() {
                flags |= StandardMaterialFlags::SPECULAR_TRANSMISSION_TEXTURE;
            }
            if self.thickness_texture.is_some() {
                flags |= StandardMaterialFlags::THICKNESS_TEXTURE;
            }
            if self.diffuse_transmission_texture.is_some() {
                flags |= StandardMaterialFlags::DIFFUSE_TRANSMISSION_TEXTURE;
            }
        }
     
        // NOTE: 0.5 is from the glTF default - do we want this?
        let mut alpha_cutoff = 0.5;
        match self.alpha_mode {
            AlphaMode::Opaque => flags |= StandardMaterialFlags::ALPHA_MODE_OPAQUE,
            AlphaMode::Mask(c) => {
                alpha_cutoff = c;
                flags |= StandardMaterialFlags::ALPHA_MODE_MASK;
            }
            AlphaMode::Blend => flags |= StandardMaterialFlags::ALPHA_MODE_BLEND,
            AlphaMode::Premultiplied => flags |= StandardMaterialFlags::ALPHA_MODE_PREMULTIPLIED,
            AlphaMode::Add => flags |= StandardMaterialFlags::ALPHA_MODE_ADD,
            AlphaMode::Multiply => flags |= StandardMaterialFlags::ALPHA_MODE_MULTIPLY,
        };

        if self.attenuation_distance.is_finite() {
            flags |= StandardMaterialFlags::ATTENUATION_ENABLED;
        }
*/

       /* let has_normal_map = self.normal_map_texture.is_some();
        if has_normal_map {
            let normal_map_id = self.normal_map_texture.as_ref().map(|h| h.id()).unwrap();
            if let Some(texture) = images.get(normal_map_id) {
                match texture.texture_format {
                    // All 2-component unorm formats
                    TextureFormat::Rg8Unorm
                    | TextureFormat::Rg16Unorm
                    | TextureFormat::Bc5RgUnorm
                    | TextureFormat::EacRg11Unorm => {
                        flags |= StandardMaterialFlags::TWO_COMPONENT_NORMAL_MAP;
                    }
                    _ => {}
                }
            }
            if self.flip_normal_map_y {
                flags |= StandardMaterialFlags::FLIP_NORMAL_MAP_Y;
            }
        }*/

        //flags |= StandardMaterialFlags::ALPHA_MODE_BLEND;
        flags |= StandardMaterialFlags::ALPHA_MODE_BLEND;
     //   flags |= StandardMaterialFlags::FOG_ENABLED;
        flags |= StandardMaterialFlags::DEPTH_MAP;  // need this ?

        flags |= StandardMaterialFlags::DOUBLE_SIDED;
     flags |= StandardMaterialFlags::UNLIT;

      //  flags |= StandardMaterialFlags::FLIP_NORMAL_MAP_Y;

       // flags |= StandardMaterialFlags::EMISSIVE_TEXTURE;

        StandardMaterialUniform {
            base_color: self.base_color.into(),
            alpha_cutoff: 0.1,
            emissive:  [0.2,0.2,0.2,0.2].into(),
            flags: flags.bits() ,
            
          
           //  roughness: 0.9,

            // From [0.0, 1.0], dielectric to pure metallic
             metallic: 0.0,

            // Specular intensity for non-metals on a linear scale of [0.0, 1.0]
            // defaults to 0.5 which is mapped to 4% reflectance in the shader
             reflectance: 0.0,
            ..default()
        }
    }
}