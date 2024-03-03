## Bevy Magic Fx 

Define mesh-based VFX in RON files and load them into bevy 

![image](https://github.com/ethereumdegen/bevy_magic_fx/assets/6249263/ad0dc5f6-e56c-4439-a417-02cbc2205e05)


### How to use 

```
cargo run --example basic
```

### Example VFX Definition 
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
### Example Shader Variant Definition 
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
