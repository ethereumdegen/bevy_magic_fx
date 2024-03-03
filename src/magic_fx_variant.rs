use std::time::Duration;

use bevy::pbr::{ExtendedMaterial, OpaqueRendererMethod};
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

use crate::animated_material::{
    self, AnimatedMaterial, AnimatedMaterialBundle, AnimatedMaterialExtension,
};

use crate::euler_transform::EulerTransform;
use crate::shader_variant::ShaderVariantManifest;

//use bevy::utils::HashMap;

//this gets loaded in

#[derive(Debug, Clone, Asset, Serialize, Deserialize)]
pub struct MagicFxVariantManifest {
    pub name: String,

    pub magic_fx_instances: Vec<MagicFxInstanceManifest>,

    pub max_time: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicFxInstanceManifest {
    pub shader_variant_name: String,

    pub mesh_name: String,
    pub start_time_offset: f32,
    pub end_time_offset: f32,
    pub start_transform: EulerTransform,
    pub end_transform: EulerTransform,
}

impl TypePath for MagicFxVariantManifest {
    fn short_type_path() -> &'static str {
        "mfxvar.ron"
    }
    fn type_path() -> &'static str {
        "mfxvar.ron"
    }
}

#[derive(Debug, Clone)]
pub struct MagicFxVariant {
    pub name: String,

    pub magic_fx_instances: Vec<MagicFxInstance>,

    // pub current_time: Duration,
    pub max_time_offset: Duration,
}

impl MagicFxVariant {
    pub fn from_manifest(
        manifest: &MagicFxVariantManifest,

        // a map of all shader variant handles which have already been loaded
        texture_handles_map: &HashMap<String, Handle<Image>>,
        mesh_handles_map: &HashMap<String, Handle<Mesh>>,
        shader_variants_map: &HashMap<String, Handle<ShaderVariantManifest>>,
        shader_variant_assets: &Res<Assets<ShaderVariantManifest>>,

        time: &Res<Time>,
    ) -> Self {
        let current_time = time.elapsed();

        Self {
            name: manifest.name.clone(),
            //current_time:  current_time ,
            max_time_offset: Duration::from_secs_f32(manifest.max_time),
            magic_fx_instances: manifest
                .magic_fx_instances
                .clone()
                .drain(..)
                .filter_map(|instance_manifest| {
                    MagicFxInstance::from_manifest(
                        instance_manifest,
                        texture_handles_map,
                        mesh_handles_map,
                        shader_variants_map,
                        shader_variant_assets,
                    )
                })
                .collect(),
        }
    }

    pub fn build_all_materials(
        mut self,
        animated_materials: &mut ResMut<Assets<AnimatedMaterialExtension>>,
    ) -> Self {
        for instance in self.magic_fx_instances.iter_mut() {
            let _bundle = &mut instance.build_material(animated_materials);
        }
        self
    }
}

#[derive(Debug, Clone)]
pub struct MagicFxInstance {
    pub shader_variant_manifest: ShaderVariantManifest,

    pub texture_handle: Handle<Image>,

    pub mesh_handle: Handle<Mesh>,
    pub start_time_offset: Duration,
    pub end_time_offset: Duration,
    pub start_transform: Transform,
    pub end_transform: Transform,

    pub shader_material: Option<Handle<AnimatedMaterialExtension>>,
}

impl MagicFxInstance {
    pub fn from_manifest(
        manifest: MagicFxInstanceManifest,

        texture_handles_map: &HashMap<String, Handle<Image>>,
        mesh_handles_map: &HashMap<String, Handle<Mesh>>,

        shader_variants_map: &HashMap<String, Handle<ShaderVariantManifest>>,
        shader_variant_manifest_assets: &Res<Assets<ShaderVariantManifest>>,
    ) -> Option<Self> {
        let mesh_handle = mesh_handles_map.get(&manifest.mesh_name).unwrap();

        let shader_variant_manifest_handle = shader_variants_map
            .get(&manifest.shader_variant_name)
            .unwrap();

        let shader_variant_manifest = shader_variant_manifest_assets
            .get(shader_variant_manifest_handle)
            .unwrap();

        let texture_handle = texture_handles_map.get(&shader_variant_manifest.texture)?; //&self.shader_variant.texture;

        Some(Self {
            texture_handle: texture_handle.clone(),

            end_time_offset: Duration::from_secs_f32(manifest.end_time_offset),
            shader_variant_manifest: shader_variant_manifest.clone(),

            shader_material: None,

            mesh_handle: mesh_handle.clone_weak(),
            start_time_offset: Duration::from_secs_f32(manifest.start_time_offset),
            start_transform: manifest.start_transform.to_transform(),
            end_transform: manifest.end_transform.to_transform(),
        })
    }

    //make this a part of shader variant ?
    //how to improve this ?
    pub fn build_material(
        &mut self,

        animated_materials: &mut ResMut<Assets<AnimatedMaterialExtension>>,
    ) -> &Self {
        let base_color = (&self.shader_variant_manifest.color).clone();
        let emissive = (&self.shader_variant_manifest.emissive).clone();

        let image_handle = &self.texture_handle;

        let shader_material = animated_materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color,
                emissive: Color::Rgba {
                    red: emissive.x,
                    green: emissive.y,
                    blue: emissive.z,
                    alpha: 1.0,
                }, //turn up bloom emission like insane
                // can be used in forward or deferred mode.
                opaque_render_method: OpaqueRendererMethod::Auto,
                alpha_mode: AlphaMode::Blend,

                ..Default::default()
            },
            extension: animated_material::AnimatedMaterial {
                base_color_texture: Some(image_handle.clone()),

                //put in more data here
                custom_uniforms: animated_material::AnimatedMaterialUniforms {
                    scroll_speed_x: self.shader_variant_manifest.animation_speed.x,
                    scroll_speed_y: self.shader_variant_manifest.animation_speed.y,
                    distortion_speed_x: self.shader_variant_manifest.distortion_speed.x,
                    distortion_speed_y: self.shader_variant_manifest.distortion_speed.y,
                    distortion_amount: self.shader_variant_manifest.distortion_amount,
                    distortion_cutoff: 1.0,
                    scroll_repeats_x: self.shader_variant_manifest.scroll_repeats.x,
                    scroll_repeats_y: self.shader_variant_manifest.scroll_repeats.y,
                    ..default()
                },
                ..default()
            },
        });

        self.shader_material = Some(shader_material.clone());

        self
    }

    pub fn to_bundle(&self) -> Option<AnimatedMaterialBundle> {
        let shader_material = &self.shader_material;

        return shader_material.as_ref().map(|shader_mat| {
            animated_material::AnimatedMaterialBundle {
                mesh: self.mesh_handle.clone(),
                material: shader_mat.clone(),

                transform: self.start_transform,
                visibility: Visibility::Hidden,

                ..default()
            }
        });
    }
}

/*

need to loop through these animated mateiral bundles and change the xform over time
*/
