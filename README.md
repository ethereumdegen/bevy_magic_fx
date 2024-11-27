## Bevy Magic Fx 

Define mesh-based VFX in RON files and load them into bevy 

![2024-03-13 18-33-39](https://github.com/ethereumdegen/bevy_magic_fx/assets/6249263/b91d55d1-dcc7-4078-9b6b-425ef7c2187a)

![beam_fx](https://github.com/user-attachments/assets/a6f15933-d6ea-4f75-a630-0c7833a83ba5)


![magic_fx_sample_1](https://github.com/ethereumdegen/bevy_magic_fx/assets/6249263/692dca74-c915-4578-b23b-041d4c83810b)

![more_vfx](https://github.com/ethereumdegen/bevy_magic_fx/assets/6249263/ba2b1827-e7e2-44ab-ae91-31e937ec375d)

### Quickstart 

```
cargo run --example preview 
```

(if it fails to load, try to delete or move all of the magic_fx_variants assets to another folder except light_sparkles.magicfx.ron. Or fix them by importing your own textures. ) 



### How to install 


1. Add this plugin to your bevy application
   
```
app .add_plugins( MagicFxPlugin )
```

2. Load and register your shader variants from files 


``` 
 		let shadvar_name = & shader_variant_manifest.name;

                    let shader_material_handle = animated_materials.add( build_animated_material(
                        shader_variant_manifest, // the ron file parsed 
                        &texture_handles_map
                        ).unwrap()
                    ); 
                    
                    asset_loading_resource.animated_material_map.insert( 
                        shadvar_name .clone(), 
                        shader_material_handle );
   
```

3.  Load and register your magic fx variants from files

```

 let magic_fx_variant_manifest: &MagicFxVariantManifest = fx_variant_assets
                        .get(&asset_handles_resource.magic_fx_variant_manifest_handle)
                        .unwrap();

                     let mesh_handles_map = &asset_loading_resource.mesh_handles_map;

                    let animated_materials_map = &asset_loading_resource.animated_material_map;
  
                    let magic_fx = MagicFxVariant::from_manifest(
                        magic_fx_variant_manifest, // the ron file parsed 
                      
                        &mesh_handles_map,
                      
                        &animated_materials_map,
                     
                        
                    ).unwrap();

			//save the variant for later spawning ..
 		asset_loading_resource.loaded_magic_fx_variants.insert( 
                        magic_fx.name.clone(), 
                        magic_fx );
      

```

4.   Spawn your magic fx variants whenever you want 


```
             let _magic_fx_root = commands
                        .spawn(SpatialBundle::default())
                        .insert(MagicFxVariantComponent {
                            magic_fx,  //this is what you saved in a resource in step 3
                            start_time: time.elapsed(),
                        })
                        .id();
```





### Example VFX Definition File (RON)
```

(
    
    name: "magic",    
       
    magic_fx_instances: [( 

     shader_variant_name: "shader_variants/purple.shadvar.ron",
	  mesh_name:  "meshes/projectile.obj", 
		  start_time_offset: 0.0,
		  end_time_offset: 3.0,
		  start_transform: (translation: (3.0,2.0,0.0), rotation:(0.0,0.0,0.0),scale:(1.0,1.0,1.0)),
		  end_transform: (translation: (4.0,0.0,0.0), rotation:(2.0,0.0,0.0),scale:(1.0,1.0,1.0)),

    )] ,  

    max_time: 5.0


)

```
### Example Shader Variant Definition File (RON)
```
(
    
    name: "purple",
    texture: "textures/fire_01.png",
    animation_speed: (0.5,0.1), // Assuming animation_speed is a string for some reason; otherwise, consider using a float or int
    distortion_speed: (0.02,0.01),
    scroll_repeats: ( 3.0,3.0),
    distortion_amount: 0.02 , 

    color: Rgba(
        red: 5,
        green: 5,
        blue: 255,
        alpha: 255 // Assuming your Color struct has rgba fields; adjust according to your actual Color struct
    ),
     emissive:  (
        5.0,20.0,5.0
    )
      
)


```



### Billboarding 

To use billboarding on meshes, you must insert a MagicFxBillboardTarget component to your camera. 
