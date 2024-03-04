//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.

use std::f32::consts::PI;

  
//use bevy::pbr::{ExtendedMaterial, OpaqueRendererMethod};
use bevy::{gltf::GltfMesh, utils::HashMap};

//use bevy::gltf::Gltf;
 

use bevy::core_pipeline::bloom::BloomSettings;

use bevy::core_pipeline::tonemapping::Tonemapping;

use bevy::{core_pipeline::bloom::BloomCompositeMode, prelude::*};

use bevy_magic_fx::magic_fx::MagicFxVariantComponent;
use bevy_magic_fx::{ MagicFxPlugin};

//use bevy_magic_fx::magic_fx::{  MagicFxVariantComponent, };

use bevy_magic_fx::animated_material::{build_animated_material, AnimatedMaterial};
use bevy_magic_fx::{
    magic_fx_variant::{MagicFxVariant, MagicFxVariantManifest},
    shader_variant::ShaderVariantManifest,
};
  




fn main() {
    App::new()
        .insert_resource(AssetHandlesResource::default())
        .insert_resource(AssetLoadingResource::default())
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(bevy_obj::ObjPlugin)


        .add_plugins( MagicFxPlugin )

        .add_systems(Update, update_loading_shader_variant_manifest)
        .add_systems(Update, update_loading_magic_fx_variant_manifest)
           

        
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)

        .run();
}

#[derive(Resource, Default)]
  struct AssetHandlesResource {
    magic_fx_variant_manifest_handle: Handle<MagicFxVariantManifest>,
    shader_variant_manifest_handle: Handle<ShaderVariantManifest>,
    // mesh_handle: Handle<Mesh>,
    // anim_material: Handle<animated_material::AnimatedMaterialExtension> ,
    // particle_texture_handle: Handle<Image>
}

#[derive(Resource, Default)]
  struct AssetLoadingResource {
    texture_handles_map: HashMap<String, Handle<Image>>,
    mesh_handles_map: HashMap<String, Handle<Mesh>>,
    shader_variants_map: HashMap<String, Handle<ShaderVariantManifest>>,

    //shader var name -> animated material 
     animated_material_map: HashMap<String, Handle<AnimatedMaterial>>,


    //name of shader variant - > built animated material
    // animated_materials_map: HashMap<String, Handle<AnimatedMaterialExtension> >,
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,

    mut asset_handles_resource: ResMut<AssetHandlesResource>,
    mut asset_loading_resource: ResMut<AssetLoadingResource>,

    mut meshes: ResMut<Assets<Mesh>>,
    // images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // custom_materials: ResMut<Assets<AnimatedMaterialExtension>>,
) {
    /*

             Simulate our bevy asset loader with 'asset_loading_resource'
    */

    let particle_texture_handle = asset_server.load("textures/fire_01.png");
    asset_loading_resource
        .texture_handles_map
        .insert("textures/fire_01.png".to_string(), particle_texture_handle);

    let shader_variant_manifest_handle = asset_server.load("shader_variants/purple.shadvar.ron");
    asset_loading_resource.shader_variants_map.insert(
        "shader_variants/purple.shadvar.ron".to_string(),
        shader_variant_manifest_handle.clone(),
    );

    asset_handles_resource.shader_variant_manifest_handle = shader_variant_manifest_handle.clone();

    let mesh_handle: Handle<Mesh> = asset_server.load("meshes/projectile.obj");
    asset_loading_resource
        .mesh_handles_map
        .insert("meshes/projectile.obj".to_string(), mesh_handle);

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
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
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
            transform: Transform::from_xyz(0.0, 6., 12.0)
                .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ..default()
        },
        BloomSettings::default(), // 2. Enable bloom for the camera
    ));
}

fn rotate(mut query: Query<&mut Transform, With<Handle<Mesh>>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

fn update_loading_shader_variant_manifest(
    mut ev_asset: EventReader<AssetEvent<ShaderVariantManifest>>,
    //  mut fx_variant_assets: ResMut<Assets<ShaderVariantManifest>>,
    mut asset_handles_resource: ResMut<AssetHandlesResource>,

    mut asset_loading_resource: ResMut<AssetLoadingResource>,
   mut animated_materials: ResMut<Assets<AnimatedMaterial>>,


    shader_variant_manifest_resource: Res<Assets<ShaderVariantManifest>>,

    asset_server: ResMut<AssetServer>,
) {
    for ev in ev_asset.read() {
        match ev {
            AssetEvent::LoadedWithDependencies { id } => {
                //once the shader variant loads, we can start loading our magic fx
                if id == &asset_handles_resource.shader_variant_manifest_handle.id() {

                     let shader_variant_manifest: &ShaderVariantManifest = shader_variant_manifest_resource
                        .get(&asset_handles_resource.shader_variant_manifest_handle)
                        .unwrap();

                    //finish loading and building the shader variant and add it to the map 
                    let texture_handles_map = &asset_loading_resource.texture_handles_map;
                


                    let shadvar_name = & shader_variant_manifest.name;

                    let shader_material_handle = animated_materials.add( build_animated_material(
                        shader_variant_manifest,
                        &texture_handles_map
                        ).unwrap()
                    ); 

                    asset_loading_resource.animated_material_map.insert( 
                        shadvar_name .clone(), 
                        shader_material_handle );



                        //now that our shadvar materials are built and loaded, we load the magic fx 
                    asset_handles_resource.magic_fx_variant_manifest_handle =
                        asset_server.load("magic_fx_variants/magic.magicfx.ron");
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
 
    asset_loading_resource: Res<AssetLoadingResource>,
 

    time: Res<Time>,
) {
    for ev in ev_asset.read() {
        match ev {
            AssetEvent::LoadedWithDependencies { id } => {
                if id == &asset_handles_resource.magic_fx_variant_manifest_handle.id() {
                   

                    let magic_fx_variant_manifest: &MagicFxVariantManifest = fx_variant_assets
                        .get(&asset_handles_resource.magic_fx_variant_manifest_handle)
                        .unwrap();

                     let mesh_handles_map = &asset_loading_resource.mesh_handles_map;

                    let animated_materials_map = &asset_loading_resource.animated_material_map;
  
                    let magic_fx = MagicFxVariant::from_manifest(
                        magic_fx_variant_manifest,
                      
                        &mesh_handles_map,
                      
                        &animated_materials_map,
                     
                        
                    ).unwrap();

                    //now we can store this in a resource 
                    println!("spawn the root ");

                    //at a later time, whenever, spawn the magic fx . This is usually from a spell cast.
                    let _magic_fx_root = commands
                        .spawn(SpatialBundle::default())
                        .insert(MagicFxVariantComponent {
                            magic_fx,
                            start_time: time.elapsed(),
                        })
                        .id();
                }
            }
            _ => {}
        }
    }
}
