use bevy::prelude::*;

pub struct CollisionsPlugin;
fn bomba_vs_dupix(
    mut commands: Commands,
    q_bomba: Query<(Entity, &Transform), With<crate::bomba::Bomba>>,
    mut q_dupix: Query<(&mut crate::movable::Movable, &Transform), With<crate::dupix::Dupix>>,
) {
    for (bomba, bomba_transform) in q_bomba.iter() {
        for (mut dupix, dupix_transform) in q_dupix.iter_mut() {
            if bomba_transform
                .translation
                .distance(dupix_transform.translation)
                < 40.
            {
                commands.entity(bomba).despawn_recursive();
                dupix.direction.y = -500.;
            }
        }
    }
}

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bomba_vs_dupix);
    }
}
