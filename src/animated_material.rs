use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;

use bevy::pbr::{ExtendedMaterial, MaterialExtension};


pub type AnimatedMaterial = ExtendedMaterial<StandardMaterial, AnimatedMaterialBase>;

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
