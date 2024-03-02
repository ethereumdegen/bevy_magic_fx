use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EulerTransform {
	pub translation: Vec3,
	pub rotation: Vec3,
	pub scale: Vec3 
}

impl EulerTransform {

  pub fn to_transform(self) -> Transform {
        Transform {
            translation: self.translation,
            rotation: Quat::from_euler(
                bevy::math::EulerRot::YXZ,
                self.rotation.y,
                self.rotation.x,
                self.rotation.z,
            ),
            scale: self.scale,
        }
    }

}