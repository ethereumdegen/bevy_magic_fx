 
 
  
 #import bevy_pbr::{
    mesh_view_bindings::globals, 
    forward_io::{VertexOutput, FragmentOutput}, 
    pbr_fragment::pbr_input_from_standard_material,
      pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
    pbr_types::STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
      pbr_deferred_functions::deferred_output
}
 #import bevy_pbr::mesh_functions
 #import bevy_pbr::prepass_utils

 #import bevy_shader_utils::fresnel::fresnel
 #import bevy_pbr::mesh_view_bindings view

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

    distortion_speed:  vec2<f32>, 
    scroll_repeats :  vec2<f32>, 
     scroll_speed :  vec2<f32>, 
   
   
    distortion_amount: f32 ,
    distortion_cutoff: f32 ,

    depth_cutoff_offset: f32 ,
    animation_frame_dimension: vec2<f32>, 
 
    current_animation_frame_index: u32,

    tint_color: vec4<f32>,


    fresnel_power: f32 ,
    //fresnel color ?
    // disturbance effect ?  

    use_masking_texture: u32, //flags?? 
    
    
};

 


@group(2) @binding(20)
var<uniform> custom_uniforms: CustomMaterialUniforms;
 
@group(2) @binding(21)
var base_color_texture: texture_2d<f32>;
@group(2) @binding(22)
var base_color_sampler: sampler;
 
 
@group(2) @binding(23)
var  masking_texture: texture_2d<f32>;
@group(2) @binding(24)
var  masking_sampler: sampler;


fn get_repeated_uv_coords(coords: vec2<f32>) -> vec2<f32> {
    let repeated_coords = vec2<f32>(
        (coords.x % (1. / f32(custom_uniforms.scroll_repeats.x))) * f32(custom_uniforms.scroll_repeats.x),
        (coords.y % (1. / f32(custom_uniforms.scroll_repeats.y))) * f32(custom_uniforms.scroll_repeats.y)
    );
    return repeated_coords;
}

 
fn get_slideshow_uv_coords(coords: vec2<f32>, anim_frame_dimension_x: u32, anim_frame_dimension_y: u32, index: u32) -> vec2<f32> {
    

    let num_layers_x = anim_frame_dimension_x;
    let num_layers_y = anim_frame_dimension_y;
    
     let layer_width = 1.0 / f32(num_layers_x);
    let layer_height = 1.0 / f32(num_layers_y);

    let x_index = index % num_layers_x;
     let y_index = index / num_layers_x;

    let x_offset = f32(x_index) * layer_width;
    let y_offset = f32(y_index) * layer_height;
    
    let slideshow_coords = vec2<f32>(
        (coords.x * layer_width) + x_offset ,
        (coords.y * layer_height) + y_offset
    );
    
    return slideshow_coords;


}




//should consider adding vertex painting to this .. need another binding of course.. performs a color shift 

 
@fragment
fn fragment(
    mesh: VertexOutput,
    @builtin(front_facing) is_front: bool,
 
  //  #ifdef MULTISAMPLED
  //      @builtin(sample_index) sample_index: u32,
  //  #endif

) ->   FragmentOutput {
    
  //  #ifndef MULTISAMPLED
        let sample_index = 0u;
  //  #endif


    let scroll_amount_x = (globals.time * custom_uniforms.scroll_speed.x)  ;
    let scroll_amount_y = (globals.time * custom_uniforms.scroll_speed.y)  ; 
 
    var tiled_uv =   get_repeated_uv_coords (mesh.uv + vec2(scroll_amount_x,scroll_amount_y)  )   ;


    if (u32(custom_uniforms.animation_frame_dimension.x) > 1u || u32(custom_uniforms.animation_frame_dimension.y) > 1u) {
        

       let current_layer_index = custom_uniforms.current_animation_frame_index;

        //this should 
        tiled_uv =  get_slideshow_uv_coords( 
         mesh.uv ,
         u32(custom_uniforms.animation_frame_dimension.x),
         u32(custom_uniforms.animation_frame_dimension.y),
         current_layer_index
         )   ;   


     }


  

      //make the cutoff big and it wont have any effect
    
    let distortion_radians_x =  (globals.time * custom_uniforms.distortion_speed.x + mesh.uv[0] * 2.0 ) % 6.28 ;
    let distortion_amount_x = ( sin(distortion_radians_x) * custom_uniforms.distortion_amount  ) % custom_uniforms.distortion_cutoff   ;
    
    let distortion_radians_y =   (globals.time * custom_uniforms.distortion_speed.y + mesh.uv[1] * 2.0 ) % 6.28 ;
    let distortion_amount_y = ( cos(distortion_radians_y) * custom_uniforms.distortion_amount  ) % custom_uniforms.distortion_cutoff  ;


    let distorted_uv = tiled_uv + vec2( distortion_amount_x, distortion_amount_y );
 
    let blended_color = textureSample(base_color_texture, base_color_sampler, distorted_uv )   ;



    //var fresnel_color = custom_uniforms.fresnel_color;
    let fresnel_power = custom_uniforms.fresnel_power;
   let fresnel_strength = 1.0;
   var fresnel = saturate (0.4 * fresnel(
    view.world_position.xyz, 
    mesh.world_position.xyz, 
    mesh.world_normal, 
    fresnel_power, 
    fresnel_strength
    ));


   
   //fresnel_color.a =   ( fresnel_color.a *  saturate(   fresnel )  );
     //mult this by fresnel color and add ? 



   
  // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(mesh, is_front);
 
    //hack the material (StandardMaterialUniform)  so the color is from the terrain splat 
  
     // alpha discard
    pbr_input.material.base_color =  pbr_input.material.base_color * blended_color ;

    var final_color = alpha_discard(pbr_input.material, pbr_input.material.base_color  )  ;

    
    
    var pbr_out: FragmentOutput;
     //only apply lighting if bit is set
       if ((pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u) {
       
            
        // pbr_input.material.base_color =  blended_color;

         pbr_out.color = apply_pbr_lighting(pbr_input);
    
         pbr_out.color = main_pass_post_lighting_processing(pbr_input, pbr_out.color);
      
          final_color = pbr_out.color;
       }  


    // -----

    //tint also affect emissive color? 

   pbr_out.color = final_color * custom_uniforms.tint_color;

   //using fresnel
    if  (fresnel_power > 0.01){
           pbr_out.color.a =  pbr_out.color.a * fresnel ; 
      }
    // pbr_out.emissive = pbr_input.material.emissive * custom_uniforms.tint_color;



     // Apply masking texture if available
    if (custom_uniforms.use_masking_texture > 0u) {
        let mask_value = textureSample(masking_texture, masking_sampler, mesh.uv ).r;
        pbr_out.color.a =  pbr_out.color.a * mask_value;  // apply mask 
    }



   var position = mesh.position; //this is frag_coord ? 

   //apply the depth_cutoff_offset
   let depth = prepass_utils::prepass_depth(position,sample_index);
   if ( (position.z - depth ) * 100.0  < custom_uniforms.depth_cutoff_offset   ) { 
       discard;
    }

       
      
 
    return pbr_out;
    
}




fn alpha_blend(top: vec4<f32>, bottom: vec4<f32>) -> vec4<f32> {
    let color = top.rgb * top.a + bottom.rgb * (1.0 - top.a);
    let alpha = top.a + bottom.a * (1.0 - top.a);
    return vec4<f32>(color, alpha);  
}
 