//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.
 

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat}, gltf::GltfMesh,
};

use bevy::gltf::Gltf;
 

use crate::custom_material::{ CustomMaterialUniforms}; 

use std::f32::consts::PI;
 

use bevy::core_pipeline::bloom::BloomSettings;
 

use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::pbr::ExtendedMaterial;
use bevy::pbr::OpaqueRendererMethod;


fn main() {
    App::new() 
      
         
         .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
         .add_plugins(MaterialPlugin::<ExtendedMaterial<StandardMaterial,custom_material::ScrollingMaterial>>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, rotate) 
        .add_systems(Update, link_enchantment_materials )
        .add_systems(Update, apply_enchantment_materials )
        .insert_resource( AssetHandlesResource::default() )
        .run();
}



pub type CustomPbrBundle = MaterialMeshBundle<ExtendedMaterial<StandardMaterial,custom_material::ScrollingMaterial>>;
mod custom_material;

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.5;


#[derive(Resource,Default)]
pub struct AssetHandlesResource {
    sword_gltf: Handle<Scene>,
    anim_material: Handle<ExtendedMaterial<StandardMaterial, custom_material::ScrollingMaterial >> 
} 

#[derive(Component)]
pub struct HasEnchantmentMaterial { 
} 
#[derive(Component,Default)]
pub struct EnchantmentMaterialLink { 

    enchantment_skin_1_material_entity: Option<Entity>,
    enchantment_skin_2_material_entity: Option<Entity>,
    enchantment_runes_material_entity: Option<Entity>,


} 





fn setup(
    mut commands: Commands,
    mut asset_server: ResMut< AssetServer>,
    
    mut asset_handles_resource: ResMut<AssetHandlesResource>,

   
     mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    mut custom_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, custom_material::ScrollingMaterial>>>,
 
) {




    let magic_texture = asset_server.load("textures/fire_01.png");
    asset_handles_resource.sword_gltf = asset_server.load("models/rpg_sword_07.glb#Scene0");

    let base_color = Color::PURPLE.set_a(0.4).clone();


    asset_handles_resource.anim_material = custom_materials.add(ExtendedMaterial {
        base: StandardMaterial {
            base_color ,
            emissive: Color::rgb_linear(50.2, 1.2, 0.8),
            // can be used in forward or deferred mode.
            opaque_render_method: OpaqueRendererMethod::Auto,
            alpha_mode: AlphaMode::Multiply,
            // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
            // in forward mode, the output can also be modified after lighting is applied.
            // see the fragment shader `extended_material.wgsl` for more info.
            // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
            // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
            ..Default::default()
        },
        extension:custom_material::ScrollingMaterial {
            base_color_texture: Some( magic_texture ),
          
            custom_uniforms: CustomMaterialUniforms{
                scroll_speed_x : 0.1,
                scroll_speed_y : 1.0,
                distortion_speed_x: 3.0,
                distortion_speed_y: 1.0,
                distortion_amount: 0.03,
                distortion_cutoff: 1.0,
                scroll_repeats_x: 12.0,
                scroll_repeats_y: 3.0,
                ..default()
            }, 
            ..default()
        },
    });
    
    
      commands.spawn( 
          SceneBundle {
              scene:  asset_handles_resource.sword_gltf.clone()  ,
                transform: Transform::from_xyz(
                                3.0,
                                2.0,
                                0.0,
                            )
                            .with_scale( (3.0,3.0,3.0).into()  )
                            ,
                          //  .with_rotation(Quat::from_rotation_x(-PI / 5.)),
                            
              ..default()
          }
               
      ).insert(HasEnchantmentMaterial{});
    //custom_materials.add();


  

   /*  let shapes = [
        meshes.add(shape::Cube::default().into()),
        meshes.add(shape::Box::default().into()),
        meshes.add(shape::Capsule::default().into()),
        meshes.add(shape::Torus::default().into()),
        meshes.add(shape::Cylinder::default().into()),
        meshes.add(shape::Icosphere::default().try_into().unwrap()),
        meshes.add(shape::UVSphere::default().into()),
    ];

   let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            CustomPbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    2.0,
                    0.0,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 40.)),
                ..default()
            },
            Shape,
            bevy::pbr::NotShadowCaster 
        ));
    }
*/

//   



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
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::SILVER.into()),
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

/*
works similar to animation linking 
*/
fn link_enchantment_materials(
    mut commands: Commands, 
    
    
    name_query:   Query<(Entity, &Name )>, 
    children_query:   Query<  &Children>, 
    
    enchanted_model_query: Query< Entity, (With<HasEnchantmentMaterial>, Without<EnchantmentMaterialLink>) >,
    
      asset_handles_resource: Res <AssetHandlesResource>,
       
      mut standard_materials: ResMut<Assets<StandardMaterial>>,  
      standard_material_query: Query<   &Handle<StandardMaterial> >,
 
){

    for enchanted_model_entity in enchanted_model_query.iter() { 
                     
                     let enchantment_layer_1_node = find_node_by_startswith_name_recursive(
                          
                         &name_query,
                         &children_query,
                      
                         enchanted_model_entity,
                         "skin_enchant_layer_1_material".into()
                     ); 
                     let enchantment_layer_2_node = find_node_by_startswith_name_recursive(
                          
                        &name_query,
                        &children_query,
                     
                        enchanted_model_entity,
                        "skin_enchant_layer_2_material".into()
                    ); 
                    let enchantment_runes_1_node = find_node_by_startswith_name_recursive(
                          
                        &name_query,
                        &children_query,
                     
                        enchanted_model_entity,
                        "skin_runes_1_material".into()
                    ); 

                    println!("inserted material link {:?}", enchantment_layer_1_node );

                    //all or nothing to prevent hard-to-find bugs
                    if enchantment_layer_1_node.is_none() {continue};
                    if enchantment_layer_2_node.is_none() {continue};
                    if enchantment_runes_1_node.is_none() {continue};

                    commands.entity(  enchanted_model_entity ).insert(
                        EnchantmentMaterialLink{
                              enchantment_skin_1_material_entity : enchantment_layer_1_node.clone() ,
                              enchantment_skin_2_material_entity : enchantment_layer_2_node.clone() ,
                              enchantment_runes_material_entity : enchantment_runes_1_node.clone() ,
                            ..default()
                        }
                      ); 
                    
            }
            
            
  
}


fn apply_enchantment_materials(
    mut commands: Commands, 
    
    
    name_query:   Query<(Entity, &Name )>, 
    children_query:   Query<  &Children>, 
    
    enchanted_model_query: Query< (Entity,&EnchantmentMaterialLink), Added<EnchantmentMaterialLink> >,
    
      asset_handles_resource: Res <AssetHandlesResource>,
       
      mut standard_materials: ResMut<Assets<StandardMaterial>>,  
      standard_material_query: Query<   &Handle<StandardMaterial> >,
 
){

    for (ench_entity,ench_mat_link) in enchanted_model_query.iter(){



        if let Some( enchantment_skin_node ) = ench_mat_link.enchantment_skin_1_material_entity { 

            if let Ok(standard_material_handle) = standard_material_query.get(enchantment_skin_node){
                
                let anim_mat_handle = &asset_handles_resource.anim_material;
                     
               

                 commands.entity(enchantment_skin_node).remove::<Handle<StandardMaterial>>( );
                 commands.entity(enchantment_skin_node).insert(anim_mat_handle.clone());
    
               
            }else{
                println!("unable to find ench  material");
                
            }
         
        }else{
           println!( "no node w that name " );
        }
       


    }

   




}


fn rotate(mut query: Query<&mut Transform , With<Handle<Mesh>>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}


fn find_node_by_startswith_name_recursive( 

    name_query:   &Query<(Entity, &Name )>, 
    children_query:   &Query<  &Children>, 
 
    current_entity: Entity,
    target_name: &str,
) -> Option<Entity> {
     
    // First, check if the current entity matches the target name
    if let Ok((entity, name)) = name_query.get(current_entity) {
        if name.as_str().starts_with( target_name ) {
            return Some(entity);
        }
        println!("found node w name {:?}", &name);
    }

    // Then, if the current entity has children, iterate over them recursively
    if let Ok(children) = children_query.get(current_entity) {
        println!("children are {:?}",children);
        for &child in children.iter() {
            if let Some(found) = find_node_by_startswith_name_recursive(name_query, children_query, child, target_name) {
                return Some(found);
            }
        }
    }

    // If no matching entity is found, return None
    None

}



// Recursive function to find a node by name in the scene graph.
/*fn find_node_by_name_recursive(
    commands: &mut Commands,
    
    name_query: &Query<&Name>,
     children_query: &Query<&Children>,
    
    current_entity: Entity,
    target_name: &str,
) -> Result<(Entity, String), &'static str> {
    if let Ok(name) = name_query.get(current_entity) {
        if name.as_str() == target_name {
            return Ok((current_entity, name.to_string()));
        }else{
            println!("found node {:?}", &name )
        }
    }

    if let Ok(children) = children_query.get(current_entity)  {
        for child in children.iter() {
            if let Ok(result) = find_node_by_name_recursive(commands,&name_query,  &children_query, *child, target_name) {
                return Ok(result);
            }
        }
    }

    Err("Node not found")
}*/