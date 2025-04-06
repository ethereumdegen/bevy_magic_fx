

/*

    code taken from bevy_mod_lookat 
    https://github.com/TotalKrill/bevy_mod_lookat/blob/main/src/lib.rs

*/
use bevy::ecs::relationship::Relationship;
use bevy::prelude::*; 

pub enum UpDirection {

    Target,
    Dir(Dir3),
    Parent, 

}


pub fn get_rotate_towards(
    rotating_entity: Entity, 

    target_entity: Entity,
    //parent: Option<Entity>, 

    up_direction:  UpDirection, 
 
    parent_query: &Query<&ChildOf>,
    global_transforms: &Query<&GlobalTransform>, // potential_targets

   // mut rotators: Query<(&mut Transform, &GlobalTransform, Option<&Parent>, &RotateTo)>, // the ones to rotate
) -> Option<Quat> {
   // for (mut rotator_t, rotator_gt, parent, target) in rotators.iter_mut() {
       let  rotator_gt  = global_transforms.get( rotating_entity ).ok() ?;
        let  target_gt  = global_transforms.get( target_entity ) .ok() ?;

        let parent_gt = if let Some(parent_e) = parent_query.get(rotating_entity).ok() {
            global_transforms.get(parent_e.get()).ok()
        } else {
            return None
        };

        let updir = match  up_direction  {
            UpDirection::Target => target_gt.up(),
            UpDirection::Dir(dir) => dir,
            UpDirection::Parent => {
                if let Some(parent_gt) = parent_gt {
                    parent_gt.up()
                } else {
                    // if there is no parent, fallback to bevy up direction
                    Dir3::Y
                }
            }
        };

        let rotation = calculate_local_rotation_to_target(rotator_gt, target_gt, parent_gt, updir);

        Some(rotation)

        //rotator_t.rotation = rotation;
   // }
}

/// Calculates the local rotation on a rotator towards a target, adjusting for rotations of eventual parents, with the selected rotator up direction.
pub fn calculate_local_rotation_to_target(
    rotator_gt: &GlobalTransform,
    target_gt: &GlobalTransform,
    parent_gt: Option<&GlobalTransform>,
    updir: Dir3,
) -> Quat {
    let target_gt_computed = target_gt.compute_transform();
    let parent_gt_computed: Option<Transform> = parent_gt.map(|p| p.compute_transform());

    let mut rotation = rotator_gt
        .compute_transform()
        .looking_at(target_gt_computed.translation, updir)
        .rotation;

    if let Some(parent_gt_computed) = parent_gt_computed {
        rotation = parent_gt_computed.rotation.inverse() * rotation;
    }
    rotation
}