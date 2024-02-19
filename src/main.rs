//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.

use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat}, gltf::GltfMesh,
};

use bevy::gltf::Gltf;


use crate::custom_material::{Repeats,CustomMaterialUniforms};


fn main() {
    App::new() 
      
         
         .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
         .add_plugins(MaterialPlugin::<custom_material::ScrollingMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .add_systems(Update, on_asset_load )
        .insert_resource( AssetHandlesResource::default() )
        .run();
}



pub type CustomPbrBundle = MaterialMeshBundle<custom_material::ScrollingMaterial>;
mod custom_material;

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.5;

#[derive(Resource,Default)]
pub struct AssetHandlesResource {
    bullet_mesh: Handle<Gltf>,
    anim_material: Handle< custom_material::ScrollingMaterial> 
}

fn setup(
    mut commands: Commands,
    mut asset_server: ResMut< AssetServer>,
    
    mut asset_handles_resource: ResMut<AssetHandlesResource>,

   
     mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<custom_material::ScrollingMaterial>>,
) {

    let magic_texture = asset_server.load("textures/spark_02.png");
    asset_handles_resource.bullet_mesh = asset_server.load("meshes/mesh_projectile.glb");

let base_color = Color::PURPLE.set_a(0.4).clone();
    asset_handles_resource.anim_material = custom_materials.add(custom_material::ScrollingMaterial {
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
        base_color ,
        ..default()
    });

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

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });
}

/*
Once our gltf loads, extract the mesh and build our bundle 
*/
fn on_asset_load(
    mut commands: Commands,
    mut ev_asset: EventReader<AssetEvent<Gltf>>,
    
       asset_handles_resource: Res <AssetHandlesResource>,
  //  mut images: ResMut<Assets<Image>>,


     gltfs: ResMut<Assets<Gltf>>,

      gltfmeshes: ResMut<Assets<GltfMesh>>,
  // mut custom_materials: ResMut<Assets<custom_material::ScrollingMaterial>>,
){

    for ev in ev_asset.read() {
        match ev {
            AssetEvent::LoadedWithDependencies { id  } => {
                
             //   let mut image_is_splat = false; 

                let loaded_handle = Handle::Weak(*id);
                
                    let bullet_mesh_handle = &asset_handles_resource.bullet_mesh;
                   
                    
                    if loaded_handle != *bullet_mesh_handle {
                        continue
                    }
                    
                     let anim_mat_handle = &asset_handles_resource.anim_material;
                    
                let custom_gltf = gltfs.get(bullet_mesh_handle).unwrap();
                let custom_mesh_handle = custom_gltf.meshes.get(0).unwrap();
                let custom_mesh = gltfmeshes.get( custom_mesh_handle ).unwrap();
                let primitive = custom_mesh.primitives.first().unwrap();
                
                
                    commands.spawn((
                        CustomPbrBundle {
                            mesh:  primitive.mesh.clone(),
                            material: anim_mat_handle.clone(),
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
            
            _ => {}
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
