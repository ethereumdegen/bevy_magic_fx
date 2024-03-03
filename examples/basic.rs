//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.

use std::f32::consts::PI;

use bevy::{
    
    gltf::GltfMesh, utils::HashMap
};

use bevy_common_assets::ron::RonAssetPlugin;
use bevy::gltf::Gltf;

use bevy::core_pipeline::bloom::BloomSettings;
 
use bevy::core_pipeline::tonemapping::Tonemapping;
 

use bevy::{
   
    core_pipeline::{
        bloom::{BloomCompositeMode},
        
    },
    prelude::*,
};
 

use bevy_magic_fx::magic_fx::{
    update_magic_fx_variants, 
    update_magic_fx_instances, 
    update_magic_fx_variants_added, 
    MagicFxVariantComponent};

use bevy_magic_fx::{ magic_fx_variant::{MagicFxVariant, MagicFxVariantManifest}, shader_variant::ShaderVariantManifest};
use bevy_magic_fx::animated_material::{self, AnimatedMaterialExtension};


use bevy_magic_fx::{magic_fx_variant,shader_variant};



fn main() {
    App::new() 
        .insert_resource( AssetHandlesResource::default() )
        .insert_resource(AssetLoadingResource::default())
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(bevy_obj::ObjPlugin)
         
        .add_plugins(RonAssetPlugin::<ShaderVariantManifest>::new(&["shadvar.ron"])) 
        .add_plugins(RonAssetPlugin::<MagicFxVariantManifest>::new(&["magicfx.ron"])) 
         
         .add_plugins(MaterialPlugin::<animated_material::AnimatedMaterialExtension>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, rotate) 
        .add_systems(Update, update_loading_shader_variant_manifest )
         .add_systems(Update, update_loading_magic_fx_variant_manifest )

         

         .add_systems( Update, update_magic_fx_variants_added  )
         .add_systems( Update, update_magic_fx_variants )
         .add_systems(Update, update_magic_fx_instances)

        .run();
}




 
#[derive(Resource,Default)]
pub struct AssetHandlesResource {
    magic_fx_variant_manifest_handle: Handle<MagicFxVariantManifest> ,
    shader_variant_manifest_handle: Handle<ShaderVariantManifest>,
    mesh_handle: Handle<Mesh>,
    anim_material: Handle<animated_material::AnimatedMaterialExtension> ,
   // particle_texture_handle: Handle<Image>
}


#[derive(Resource,Default)]
pub struct AssetLoadingResource {

    texture_handles_map: HashMap<String,Handle<Image>>,
    mesh_handles_map: HashMap<String,Handle<Mesh>>,
    shader_variants_map: HashMap<String, Handle<ShaderVariantManifest> >, 
        
        //name of shader variant - > built animated material 
    animated_materials_map: HashMap<String, Handle<AnimatedMaterialExtension> >, 
              


}



fn setup(
    mut commands: Commands,
    mut asset_server: ResMut< AssetServer>,
    
    mut asset_handles_resource: ResMut<AssetHandlesResource>,
    mut asset_loading_resource: ResMut<AssetLoadingResource>,

   
     mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    mut custom_materials: ResMut<Assets<AnimatedMaterialExtension>>,
 
) {

  
   /*

            Simulate our bevy asset loader with 'asset_loading_resource'
   */

    let particle_texture_handle = asset_server.load("textures/fire_01.png");
    asset_loading_resource.texture_handles_map.insert("textures/fire_01.png".to_string(),particle_texture_handle);
   

    let shader_variant_manifest_handle = asset_server.load("shader_variants/purple.shadvar.ron");
    asset_loading_resource.shader_variants_map.insert("shader_variants/purple.shadvar.ron".to_string(),shader_variant_manifest_handle.clone_weak());

    asset_handles_resource.shader_variant_manifest_handle = shader_variant_manifest_handle.clone();
    

    let mesh_handle:Handle<Mesh> = asset_server.load("meshes/projectile.obj");
    asset_loading_resource.mesh_handles_map.insert("meshes/projectile.obj".to_string(),mesh_handle);

 


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
 



fn update_loading_shader_variant_manifest(
      mut ev_asset: EventReader<AssetEvent<ShaderVariantManifest>>,
    //  mut fx_variant_assets: ResMut<Assets<ShaderVariantManifest>>,

     mut asset_handles_resource: ResMut<AssetHandlesResource>,
        asset_server: ResMut< AssetServer>,
    


    ){
     for ev in ev_asset.read() {
            match ev {
                AssetEvent::LoadedWithDependencies { id } => {

                        //once the shader variant loads, we can start loading our magic fx 
                    if id == &asset_handles_resource.shader_variant_manifest_handle.id() { 

                         asset_handles_resource.magic_fx_variant_manifest_handle = asset_server.load("magic_fx_variants/magic.magicfx.ron");
                    }


            }
        _ => {} 
        }
    }

}



fn update_loading_magic_fx_variant_manifest(
    mut ev_asset: EventReader<AssetEvent<MagicFxVariantManifest>>,
      fx_variant_assets: ResMut<Assets<MagicFxVariantManifest>>,

    mut commands: Commands,
   
      asset_handles_resource: ResMut<AssetHandlesResource>,

  
      shader_variant_assets: Res<Assets<ShaderVariantManifest>>,

     asset_loading_resource: Res <AssetLoadingResource>,

   mut animated_materials:   ResMut<Assets<AnimatedMaterialExtension>>,

   time: Res<Time>
 
   
) {
    for ev in ev_asset.read() {
        match ev {
            AssetEvent::LoadedWithDependencies { id } => {

                if id == &asset_handles_resource.magic_fx_variant_manifest_handle.id() {
 
                  let magic_fx_variant_manifest:&MagicFxVariantManifest = fx_variant_assets.get( &asset_handles_resource.magic_fx_variant_manifest_handle ).unwrap();
 

                  let  texture_handles_map = &asset_loading_resource.texture_handles_map;
                  let  mesh_handles_map = &asset_loading_resource.mesh_handles_map;

                  let  shader_variants_map = &asset_loading_resource.shader_variants_map;

                   

                   let   magic_fx = MagicFxVariant::from_manifest(
                        magic_fx_variant_manifest, 
                        &texture_handles_map,
                        &mesh_handles_map,
                        &shader_variants_map,
                        &shader_variant_assets,
                        &time
                    ).build_all_materials( &mut animated_materials ); 



                   let _magic_fx_root = commands.spawn(  SpatialBundle::default() )
                        .insert( MagicFxVariantComponent {
                            magic_fx,
                            start_time: time.elapsed()
                        }    )
                        .id();



                       

                    } 

            }
            _ => {} 
            
        }
    }
}
