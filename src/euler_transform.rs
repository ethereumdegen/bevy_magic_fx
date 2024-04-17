use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EulerTransform {
    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl EulerTransform {
    pub fn to_transform(self) -> Transform {

        
  
        Transform {
            translation: self.translation,
           rotation: Quat::from_euler(  //this is truncating or finding the shortest path! wrong 
                bevy::math::EulerRot::YXZ,
                self.rotation.x,
                self.rotation.y,
                self.rotation.z,
            ),
            scale: self.scale,
        }
    }
}
