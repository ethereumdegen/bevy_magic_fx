//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.

use std::f32::consts::PI;

use bevy::{
    
    render::render_resource::{Extent3d, TextureDimension, TextureFormat}, gltf::GltfMesh,
};

use bevy_common_assets::ron::RonAssetPlugin;
use bevy::gltf::Gltf;

use bevy::core_pipeline::bloom::BloomSettings;
 
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::pbr::ExtendedMaterial;
use bevy::pbr::OpaqueRendererMethod;

use bevy::{
   
    core_pipeline::{
        bloom::{BloomCompositeMode},
        
    },
    prelude::*,
};
 

use bevy_magic_fx::{magic_fx_variant::{MagicFxVariant, MagicFxVariantManifest}, shader_variant::ShaderVariantManifest};
use bevy_magic_fx::animated_material::{self, AnimatedMaterialExtension};


use bevy_magic_fx::{magic_fx_variant,shader_variant};



fn main() {
    App::new() 
       
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(bevy_obj::ObjPlugin)
         
        .add_plugins(RonAssetPlugin::<ShaderVariantManifest>::new(&["shadvar.ron"])) 
        .add_plugins(RonAssetPlugin::<MagicFxVariantManifest>::new(&["magicfx.ron"])) 
         
         .add_plugins(MaterialPlugin::<animated_material::AnimatedMaterialExtension>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, rotate) 
        .add_systems(Update, update_loading_magic_fx_variant_manifest )
        .insert_resource( AssetHandlesResource::default() )
        .run();
}




 
#[derive(Resource,Default)]
pub struct AssetHandlesResource {
    magic_fx_variant_manifest_handle: Handle<MagicFxVariantManifest> ,
    shader_variant_manifest_handle: Handle<ShaderVariantManifest>,
    mesh_handle: Handle<Mesh>,
    anim_material: Handle<animated_material::AnimatedMaterialExtension> ,
    particle_texture_handle: Handle<Image>
}



fn setup(
    mut commands: Commands,
    mut asset_server: ResMut< AssetServer>,
    
    mut asset_handles_resource: ResMut<AssetHandlesResource>,

   
     mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    mut custom_materials: ResMut<Assets<AnimatedMaterialExtension>>,
 
) {

   // let magic_fx_variant_manifest_handle:Handle<MagicFxVariantManifest> = asset_server.load("magic_fx_variants/magic.magicfx.ron");
   // let shader_variant_manifest_handle:Handle<ShaderVariantManifest> = asset_server.load("shader_variants/purple.shadvar.ron");

    asset_handles_resource.magic_fx_variant_manifest_handle = asset_server.load("magic_fx_variants/magic.magicfx.ron");
    asset_handles_resource.shader_variant_manifest_handle = asset_server.load("shader_variants/purple.shadvar.ron");

    asset_handles_resource.particle_texture_handle = asset_server.load("textures/fire_01.png");
    asset_handles_resource.mesh_handle = asset_server.load("meshes/projectile.obj");

   // let base_color = Color::PURPLE.set_a(0.4).clone();

/*
    asset_handles_resource.anim_material = custom_materials.add(ExtendedMaterial {
        base: StandardMaterial {
            base_color ,
            emissive: Color::rgb_linear(500.2, 3000.2, 200.8),  //turn up bloom emission like insane 
            // can be used in forward or deferred mode.
            opaque_render_method: OpaqueRendererMethod::Auto,
            alpha_mode: AlphaMode::Blend,
            
            ..Default::default()
        },
        extension:animated_material::AnimatedMaterial {
            base_color_texture: Some( magic_texture ),
          
            custom_uniforms: animated_material::AnimatedMaterialUniforms{
                scroll_speed_x : 0.4,
                scroll_speed_y : 1.0,
                distortion_speed_x: 3.0,
                distortion_speed_y: 9.0,
                distortion_amount: 0.09,
                distortion_cutoff: 1.0,
                scroll_repeats_x: 12.0,
                scroll_repeats_y: 3.0,
                ..default()
            }, 
            ..default()
        },
    });
    
    
    
    
                     let bullet_mesh_handle = &asset_handles_resource.bullet_mesh;
                   
                     let anim_mat_handle:&Handle<animated_material::AnimatedMaterialExtension> = &asset_handles_resource.anim_material;
                    
               
          

                    commands.spawn((
                        animated_material::AnimatedMaterialBundle {
                            mesh:  bullet_mesh_handle.clone(),
                            material:  anim_mat_handle.clone(),
                          
                            transform: Transform::from_xyz(
                                3.0,
                                2.0,
                                0.0,
                            )
                            .with_rotation(Quat::from_rotation_x(-PI / 5.)),
                            ..default()
                        },
                        
                        bevy::pbr::NotShadowCaster 
                    ));
                    
                    
    */
    //custom_materials.add();


   


    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add( Plane3d::default().mesh().size(50.0, 50.0) ),
          material: materials.add(Color::SILVER),
        ..default()
    });
 

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true, // 1. HDR must be enabled on the camera
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ..default()
        },
        BloomSettings::default(), // 2. Enable bloom for the camera
    ));


}
 


fn rotate(mut query: Query<&mut Transform , With<Handle<Mesh>>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}
 

fn update_loading_magic_fx_variant_manifest(
    mut ev_asset: EventReader<AssetEvent<MagicFxVariantManifest>>,
    mut fx_variant_assets: ResMut<Assets<MagicFxVariantManifest>>,
   // map_img: Res<MyMapImage>,

    mut asset_handles_resource: ResMut<AssetHandlesResource>,

    asset_server: Res<AssetServer> ,

   
) {
    for ev in ev_asset.read() {
        match ev {
            AssetEvent::LoadedWithDependencies { id } => {

                if id == &asset_handles_resource.magic_fx_variant_manifest_handle.id() {


                    let magic_fx_variant:&MagicFxVariantManifest = fx_variant_assets.get( &asset_handles_resource.magic_fx_variant_manifest_handle ).unwrap();

                    //spawn it 


                    let magic_fx = MagicFxVariant::from_manifest(magic_fx_variant,&asset_server); 

                    for instance in  &magic_fx_variant.magic_fx_instances{





                    asset_handles_resource.anim_material = animated_materials.add(ExtendedMaterial {
                        base: StandardMaterial {
                            base_color ,
                            emissive: Color::rgb_linear(500.2, 3000.2, 200.8),  //turn up bloom emission like insane 
                            // can be used in forward or deferred mode.
                            opaque_render_method: OpaqueRendererMethod::Auto,
                            alpha_mode: AlphaMode::Blend,
                            
                            ..Default::default()
                        },
                        extension:animated_material::AnimatedMaterial {
                            base_color_texture: Some( magic_texture ),
                          
                            custom_uniforms: animated_material::AnimatedMaterialUniforms{
                                scroll_speed_x : 0.4,
                                scroll_speed_y : 1.0,
                                distortion_speed_x: 3.0,
                                distortion_speed_y: 9.0,
                                distortion_amount: 0.09,
                                distortion_cutoff: 1.0,
                                scroll_repeats_x: 12.0,
                                scroll_repeats_y: 3.0,
                                ..default()
                            }, 
                            ..default()
                        },
                    });




                    commands.spawn((
                        animated_material::AnimatedMaterialBundle {
                            mesh:  bullet_mesh_handle.clone(),
                            material:  anim_mat_handle.clone(),
                          
                            transform: Transform::from_xyz(
                                3.0,
                                2.0,
                                0.0,
                            )
                            .with_rotation(Quat::from_rotation_x(-PI / 5.)),
                            ..default()
                        },
                        
                        bevy::pbr::NotShadowCaster 
                    ));

                }




                    }




            }
            _ => {} 
            
        }
    }
}
