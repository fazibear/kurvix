use bevy::prelude::*;

pub struct CollisionsPlugin;
fn bomba_vs_dupix(
    mut commands: Commands,
    q_bomba: Query<(Entity, &Transform), With<crate::bomba::Bomba>>,
    mut q_dupix: Query<
        (
            &mut Transform,
            &mut crate::movable::Movable,
            &mut crate::animable::Animable,
        ),
        (With<crate::dupix::Dupix>, Without<crate::bomba::Bomba>),
    >,
) {
    for (bomba, bomba_transform) in q_bomba.iter() {
        for (mut dupix_transform, mut dupix_movable, mut dupix_animable) in q_dupix.iter_mut() {
            if bomba_transform
                .translation
                .distance(dupix_transform.translation)
                < 40.
            {
                commands.entity(bomba).despawn_recursive();
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
