 
//see bindings in terrain_material.rs 
  
 #import bevy_pbr::{
    mesh_view_bindings::globals, 
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::alpha_discard,
    pbr_fragment::pbr_input_from_standard_material,
      pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
    pbr_types::STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
}
  
struct StandardMaterial {
    time: f32,
    base_color: vec4<f32>,
    emissive: vec4<f32>,
    perceptual_roughness: f32,
    metallic: f32,
    reflectance: f32,
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32,
    alpha_cutoff: f32,
};
 
struct CustomMaterialUniforms {
    animation_speed_multiplier: f32,
    animation_style: u32 
    
};

//https://github.com/DGriffin91/bevy_mod_standard_material/blob/main/assets/shaders/pbr.wgsl

/*
@group(1) @binding(1)
var base_color_texture_1: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler_1: sampler;
 */



struct Repeats {
    horizontal: u32,
    vertical: u32,
}


@group(1) @binding(3)
var emissive_texture: texture_2d<f32>;
@group(1) @binding(4)
var emissive_sampler: sampler;

@group(1) @binding(5)
var metallic_roughness_texture: texture_2d<f32>;
@group(1) @binding(6)
var metallic_roughness_sampler: sampler;

@group(1) @binding(7)
var occlusion_texture: texture_2d<f32>;
@group(1) @binding(8)
var occlusion_sampler: sampler;


@group(1) @binding(20)
var<uniform> custom_uniforms: CustomMaterialUniforms;
 
@group(1) @binding(21)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(22)
var base_color_sampler: sampler;
 
 @group(1) @binding(23)
var<uniform> repeats: Repeats;



fn get_repeated_uv_coords(coords: vec2<f32>) -> vec2<f32> {
    let repeated_coords = vec2<f32>(
        (coords.x % (1. / f32(repeats.horizontal))) * f32(repeats.horizontal),
        (coords.y % (1. / f32(repeats.vertical))) * f32(repeats.vertical)
    );
    return repeated_coords;
}


//should consider adding vertex painting to this .. need another binding of course.. performs a color shift 

@fragment
fn fragment(
    mesh: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    
    let scroll_amount = (globals.time * custom_uniforms.animation_speed_multiplier)  ;
 
   let tiled_uv =   get_repeated_uv_coords (mesh.uv + vec2(scroll_amount,scroll_amount));
     
    
    //this technique lets us use 255 total textures BUT we can only layer 2 at a time.  
    let color_from_texture_0 = textureSample(base_color_texture, base_color_sampler, tiled_uv );
 
    let blended_color = color_from_texture_0   ;


   
  // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(mesh, is_front);
 
    //hack the material (StandardMaterialUniform)  so the color is from the terrain splat 
    pbr_input.material.base_color =  blended_color;


    var final_color = pbr_input.material.base_color  ;

 
     //only apply lighting if bit is set
       if ((pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u) {
          var pbr_out: FragmentOutput;
            
         pbr_out.color = apply_pbr_lighting(pbr_input);
    
         pbr_out.color = main_pass_post_lighting_processing(pbr_input, pbr_out.color);
      
          final_color = pbr_out.color;
       }  


    // -----

   

    
      
 
    return final_color;
    
}
 