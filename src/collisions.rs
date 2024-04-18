use bevy::prelude::*;

use crate::animable::Animable;
use crate::bomba::Bomba;
use crate::dupix::Dupix;
use crate::info::Info;
use crate::movable::Movable;

pub struct CollisionsPlugin;
fn bomba_vs_dupix(
    mut commands: Commands,
    mut info: Query<&mut Info>,
    q_bomba: Query<(Entity, &Transform), With<Bomba>>,
    mut q_dupix: Query<
        (&mut Transform, &mut Movable, &mut Animable),
        (With<Dupix>, Without<Bomba>),
    >,
) {
    let mut info = info.single_mut();
    for (bomba, bomba_transform) in q_bomba.iter() {
        for (mut dupix_transform, mut dupix_movable, mut dupix_animable) in q_dupix.iter_mut() {
            if bomba_transform
                .translation
                .distance(dupix_transform.translation)
                < 40.
            {
                commands.entity(bomba).despawn_recursive();
                info.points += 1;
                dupix_movable.direction.y = -450.;
                dupix_transform.rotation.z = -0.3;
                dupix_animable.start_frame = 4;
                dupix_animable.end_frame = 5;
            }
        }
    }
}

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bomba_vs_dupix);
    }
}
