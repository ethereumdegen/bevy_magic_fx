use std::time::Duration;

use bevy::prelude::*;

use crate::{
    magic_fx_variant::{MagicFxInstance, MagicFxVariant},
    util::lerp_transforms,
};

#[derive(Component)]
pub struct MagicFxVariantComponent {
    pub magic_fx: MagicFxVariant,
    pub start_time: Duration,
}

#[derive(Component)]
pub struct MagicFxInstanceComponent {
    pub instance: MagicFxInstance,
    pub start_time: Duration,
}

/*

build plugins for the above comps

*/

pub fn update_magic_fx_variants_added(
    mut commands: Commands,
    magic_fx_query: Query<(Entity, &MagicFxVariantComponent), Added<MagicFxVariantComponent>>,
    time: Res<Time>,
) {
    for (fx_entity, magic_fx_comp) in magic_fx_query.iter() {
        let magic_fx = &magic_fx_comp.magic_fx;

        for instance in magic_fx.magic_fx_instances.iter() {
            println!("spawn magic fx instance!!");

            let bundle = &instance.to_bundle();

            let magic_fx_child = commands
                .spawn((
                    bundle.clone(),
                    MagicFxInstanceComponent {
                        instance: instance.clone(),
                        start_time: time.elapsed(),
                    },
                    bevy::pbr::NotShadowCaster,
                ))
                .id();

            commands.entity(fx_entity).add_child(magic_fx_child);
        }
    }
}

pub fn update_magic_fx_variants(
    mut commands: Commands,
    magic_fx_query: Query<(Entity, &MagicFxVariantComponent)>,
    time: Res<Time>,
) {
    let current_time = time.elapsed();

    for (fx_entity, magic_fx_comp) in magic_fx_query.iter() {
        let magic_fx = &magic_fx_comp.magic_fx;

        if current_time > magic_fx_comp.start_time + magic_fx.max_time_offset {
            commands.entity(fx_entity).despawn_recursive();
        }
    }
}

pub fn update_magic_fx_instances(
    mut magic_fx_query: Query<(
        Entity,
        &mut Visibility,
        &mut Transform,
        &MagicFxInstanceComponent,
    )>,
    time: Res<Time>,
) {
    let current_time = time.elapsed();

    for (entity, mut fx_visibility, mut fx_xform, instance_comp) in magic_fx_query.iter_mut() {
        let instance = &instance_comp.instance;
        let start_time = instance_comp.start_time + instance.start_time_offset;
        let end_time = instance_comp.start_time + instance.end_time_offset;

        let start_xform = instance.start_transform;
        let end_xform = instance.end_transform;

        let is_visible = current_time > start_time && current_time < end_time;

        *fx_visibility = match is_visible {
            true => Visibility::Inherited,
            false => Visibility::Hidden,
        };

        if is_visible {
            let duration = end_time.as_secs_f32() - start_time.as_secs_f32();
            let elapsed = current_time.as_secs_f32() - start_time.as_secs_f32();

            let lerp_amount: f32 = if duration > 0.0 {
                elapsed / duration
            } else {
                0.0
            }
            .clamp(0.0, 1.0);

            // Assuming `Transform` struct has a `lerp` function
            *fx_xform = lerp_transforms(&start_xform, &end_xform, lerp_amount);
        } else {
            // If not visible, or outside the lerp range, you might want to set it to start or end transform
            // Adjust this based on your needs, for now, let's assume it resets to start_xform if not visible
            *fx_xform = start_xform;
        }

        println!("update visibility {:?}", is_visible);
        //let bundle =  &mut instance.to_bundle() ;
    }
}
