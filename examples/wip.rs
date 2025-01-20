//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.

use bevy_magic_fx::magic_fx_beam::MagicFxBeamComponent;
use std::f32::consts::PI;

  
use bevy::asset::{AssetPath, LoadedFolder};
use bevy::core_pipeline::prepass::DepthPrepass;
//use bevy::pbr::{ExtendedMaterial, OpaqueRendererMethod};
use bevy::{gltf::GltfMesh, utils::HashMap};

//use bevy::gltf::Gltf;
 

use bevy::core_pipeline::bloom::{Bloom };

use bevy::core_pipeline::tonemapping::Tonemapping;

use bevy::{core_pipeline::bloom::BloomCompositeMode, prelude::*};

use bevy_magic_fx::magic_fx::{MagicFxNoAutoTransform,MagicFxVariantComponent,MagicFxBillboardTarget};
use bevy_magic_fx::{ MagicFxPlugin};

//use bevy_magic_fx::magic_fx::{  MagicFxVariantComponent, };

use bevy_magic_fx::animated_material::{build_animated_material, AnimatedMaterial};
use bevy_magic_fx::{
    magic_fx_variant::{MagicFxVariant, MagicFxVariantManifest},
    shader_variant::ShaderVariantManifest,
};
  

use bevy_magic_fx::camera;


fn main() {
    App::new()
   
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(bevy_obj::ObjPlugin)

             .insert_resource(BuiltVfxResource::default())
        .insert_resource(AssetLoadingResource::default())
        .insert_resource(FolderLoadingResource::default())
         .init_state::<LoadingState>()


        .add_plugins( MagicFxPlugin )


         .add_systems(Update, update_load_folders)
 

        .add_systems(OnEnter(LoadingState::FundamentalAssetsLoad), update_loading_shader_variant_manifest)
        .add_systems(OnEnter(LoadingState::ShadersLoad), update_loading_magic_fx_variant_manifest)
         .add_systems(OnEnter(LoadingState::Complete) , spawn_magic_fx) 

        
        .add_systems(Startup, setup)
        .add_systems(Update, camera::update_camera_look)
        .add_systems(Update, camera::update_camera_move)

        .run();
}
 



#[derive(Resource, Default)]
  struct BuiltVfxResource {


    magic_fx_variants: HashMap<String, MagicFxVariant>      

}




#[derive(Resource, Default)]
  struct AssetLoadingResource {
    texture_handles_map: HashMap<String, Handle<Image>>,
    mesh_handles_map: HashMap<String, Handle<Mesh>>,
    shader_variants_map: HashMap<String, Handle<ShaderVariantManifest>>,

    magic_fx_variants_map: HashMap<String, Handle<MagicFxVariantManifest>>,

    
     animated_material_map: HashMap<String, Handle<AnimatedMaterial>>,
 
}


#[derive(Resource, Default)]
  struct FolderLoadingResource {
   

    textures_folder_handle: Handle<LoadedFolder>,
    shadvars_folder_handle: Handle<LoadedFolder>,
    meshes_folder_handle: Handle<LoadedFolder>,

      magicfx_folder_handle: Handle<LoadedFolder>,

}

/*
#[derive(Event)]
pub enum LoadStateEvent {

    FundamentalAssetsLoaded 

}

*/

#[derive(States,Hash,Eq,PartialEq,Debug,Clone,Default)]
pub enum LoadingState {
    #[default]
    Init,
    FundamentalAssetsLoad,
    ShadersLoad,
    Complete

}



fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
 

    mut folder_loading_resource: ResMut<FolderLoadingResource>,

    mut meshes: ResMut<Assets<Mesh>>,
    
    mut materials: ResMut<Assets<StandardMaterial>>,
     
) {
    /*

             Simulate our bevy asset loader with 'asset_loading_resource'
    */

    let textures_folder = asset_server.load_folder("textures/");

    let shadvars_folder = asset_server.load_folder("shader_variants/");

     let meshes_folder = asset_server.load_folder("meshes/");


      let magic_fx_variants_folder = asset_server.load_folder("magic_fx_variants/");


     folder_loading_resource.textures_folder_handle = textures_folder;
     folder_loading_resource.shadvars_folder_handle = shadvars_folder;
     folder_loading_resource.meshes_folder_handle = meshes_folder;
     folder_loading_resource.magicfx_folder_handle = magic_fx_variants_folder;

 
    commands.spawn(


        (

            PointLight {
                intensity: 2000.0,
                range: 100.,
                shadows_enabled: false,
                ..default()
          },


            Transform::from_xyz(8.0, 16.0, 8.0)


            )

        );

     commands.insert_resource(AmbientLight {
        color: Color::linear_rgba(1.0, 1.0, 1.0, 1.0),
        brightness: 4000.0,
    });

     let silver_color = Color::linear_rgba(0.22, 0.22, 0.22, 0.2);

    // ground plane
    commands.spawn(


        (
            Mesh3d(  meshes.add(Plane3d::default().mesh().size(50.0, 50.0)) ) ,
             MeshMaterial3d(  materials.add( silver_color ) )
        )

     );

    commands.spawn((

        Camera3d::default() , 

        Camera {
                hdr: true, // 1. HDR must be enabled on the camera
                ..default()
            },

            Tonemapping::TonyMcMapface,

             Transform::from_xyz(0.0, 6., 12.0)
                .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),


       
      //  BloomSettings::default(), // 2. Enable bloom for the camera

         Bloom::OLD_SCHOOL,
        DepthPrepass,
        MagicFxBillboardTarget {},
    ));
}
 


fn update_load_folders(
       mut ev_asset: EventReader<AssetEvent<LoadedFolder>>,

       asset_server: ResMut<AssetServer>,

       loaded_folder_assets: Res<Assets<LoadedFolder>>,

      mut asset_loading_resource: ResMut<AssetLoadingResource>,

      mut next_state: ResMut<NextState<LoadingState>>

    ){


  for ev in ev_asset.read() {
        match ev {
            AssetEvent::LoadedWithDependencies { id } => {
             
            let loaded_folder = loaded_folder_assets.get( *id  ).unwrap();  


            for handle in &loaded_folder.handles {
                let asset_path = asset_server.get_path( handle.id()  ).unwrap(); 

                info!("asset path {:?}", asset_path); 

              
                if (&asset_path.path()).starts_with("meshes") { 
                         asset_loading_resource.mesh_handles_map.insert((&asset_path.path().to_str().unwrap().to_string()).clone(), asset_server.load(  &asset_path ) ) ;
                }
 
                if (&asset_path.path()).starts_with("textures") { 
                         asset_loading_resource.texture_handles_map.insert((&asset_path.path().to_str().unwrap().to_string()).clone(), asset_server.load(  &asset_path ) ) ;
                }

                if (&asset_path.path()).starts_with("shader_variants") { 
                         asset_loading_resource.shader_variants_map.insert((&asset_path.path().to_str().unwrap().to_string()).clone(), asset_server.load(  &asset_path ) ) ;
                }

                  if (&asset_path.path()).starts_with("magic_fx_variants") { 
                         asset_loading_resource.magic_fx_variants_map.insert((&asset_path.path().to_str().unwrap().to_string()).clone(), asset_server.load(  &asset_path ) ) ;
                }

               
            }


            if ! asset_loading_resource.mesh_handles_map.is_empty() 
            &&  !asset_loading_resource.texture_handles_map.is_empty()
            &&  !asset_loading_resource.shader_variants_map.is_empty() 
             &&  !asset_loading_resource.magic_fx_variants_map.is_empty() 

            {

                next_state.set(LoadingState::FundamentalAssetsLoad);
            }




         }
         _ => {} 


     }

    }

}

fn update_loading_shader_variant_manifest(
    
     

    mut asset_loading_resource: ResMut<AssetLoadingResource>,
    mut animated_materials: ResMut<Assets<AnimatedMaterial>>,


    shader_variant_manifest_resource: Res<Assets<ShaderVariantManifest>>,

    asset_server: ResMut<AssetServer>,

     mut next_state: ResMut<NextState<LoadingState>>,
) {
     
                //once the shader variant loads, we can start loading our magic fx

                for (file_path, shader_manifest_handle) in asset_loading_resource.shader_variants_map.clone().iter() {
             

                     let shader_variant_manifest: &ShaderVariantManifest = shader_variant_manifest_resource
                        .get( shader_manifest_handle.id())
                        .unwrap();

                    //finish loading and building the shader variant and add it to the map 
                    let texture_handles_map = &asset_loading_resource.texture_handles_map;
                    

                        let file_path_clone = file_path.clone();
                    let shadvar_name = AssetPath::parse(file_path_clone.as_str()).path().file_stem().unwrap().to_str().unwrap().to_string()  ;

                    let shader_material_handle = animated_materials.add( build_animated_material(
                        shader_variant_manifest,
                        &texture_handles_map
                        ).expect(format!("Could not load {:?}", &shadvar_name).as_str())
                    ); 
                    println!("adding shadvar_name {:?}",&shadvar_name);

                    asset_loading_resource.animated_material_map.insert( 
                         shadvar_name , 
                        shader_material_handle );

                  // 

                  if asset_loading_resource.animated_material_map.len() 
                     >= asset_loading_resource.shader_variants_map.len() {
                        info!("shaders load ");
                                next_state.set(LoadingState::ShadersLoad);
                   }
                    

                  /* if asset_loading_resource.animated_material_map.clone().into_values().len()   >= 1 {
                       asset_handles_resource.magic_fx_variant_manifest_handle =
                          asset_server.load("magic_fx_variants/waterfall.magicfx.ron");

                   }*/
                        //now that our shadvar materials are built and loaded, we load the magic fx 
                    
                   
                }
       
}

fn update_loading_magic_fx_variant_manifest(
   // mut ev_asset: EventReader<AssetEvent<MagicFxVariantManifest>>,
    fx_variant_assets: ResMut<Assets<MagicFxVariantManifest>>,

    mut commands: Commands,

   mut built_vfx_resource: ResMut<BuiltVfxResource>,
 
    asset_loading_resource: Res<AssetLoadingResource>,

    mut next_state: ResMut<NextState<LoadingState>>,
 

    animated_materials_assets: Res<Assets<AnimatedMaterial>>,
    mut asset_server: ResMut<AssetServer>,


) {
     

       info!("update_loading_magic_fx_variant_manifest ");


           for (file_path, magic_fx_handle) in asset_loading_resource.magic_fx_variants_map.clone().iter() {
                   

            let magic_fx_variant_manifest: &MagicFxVariantManifest = fx_variant_assets
                        .get( magic_fx_handle.id() )
                        .unwrap();



                     let mesh_handles_map = &asset_loading_resource.mesh_handles_map;

                    let animated_materials_map = &asset_loading_resource.animated_material_map;
  
                    let magic_fx = MagicFxVariant::from_manifest(
                        magic_fx_variant_manifest,
                      
                        &mesh_handles_map,
                      
                        &animated_materials_map,


                        &animated_materials_assets,
                        &mut asset_server 
     

                     
                        
                    ).expect(format!("could not load {:?}",file_path.to_string()).as_str());
                    info!("loaded {:?}",file_path.to_string());
                    built_vfx_resource.magic_fx_variants.insert(file_path.to_string(), magic_fx);

                   
                 
        }


           next_state.set(LoadingState::Complete);
}


fn spawn_magic_fx(
    mut commands: Commands, 
     built_vfx_resource: Res <BuiltVfxResource>,

     time: Res<Time>
    ){

          println!("spawning magic fx  ");
            let waterfall_fx = built_vfx_resource.magic_fx_variants.get("magic_fx_variants/wip/soul_fire.magicfx.ron").unwrap();
 

          //at a later time, whenever, spawn the magic fx . This is usually from a spell cast.
              commands  .spawn(

                            (Transform::from_xyz(0.0,0.0,0.0), Visibility::default( ) )
                            )
                        .insert(MagicFxVariantComponent {
                            magic_fx: waterfall_fx.clone(),
                            start_time: time.elapsed(),
                        }) ;

            /*
         let spellcast1_fx = built_vfx_resource.magic_fx_variants.get("magic_fx_variants/spellcast1.magicfx.ron").unwrap();

          commands .spawn(   (Transform::from_xyz(2.0,0.0,0.0), Visibility::default( ) ) )
                        .insert(MagicFxVariantComponent {
                            magic_fx: spellcast1_fx.clone(),
                            start_time: time.elapsed(),
                        }) ;



         let smoke_poof_fx = built_vfx_resource.magic_fx_variants.get("magic_fx_variants/smoke_poof.magicfx.ron").unwrap();

          commands .spawn(   (Transform::from_xyz(4.0,0.0,0.0), Visibility::default( ) ) )
                        .insert(MagicFxVariantComponent {
                            magic_fx: smoke_poof_fx.clone(),
                            start_time: time.elapsed(),
                        }) ;

         */


}
