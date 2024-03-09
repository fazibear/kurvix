use bevy::prelude::*;

pub struct CollisionsPlugin;
fn bomba_vs_dupix(
    mut commands: Commands,
    q_bomba: Query<(Entity, &Transform), With<crate::bomba::Bomba>>,
    q_dupix: Query<(Entity, &Transform), With<crate::dupix::Dupix>>,
) {
    for (bomba, bomba_transform) in q_bomba.iter() {
        for (dupix, dupix_transform) in q_dupix.iter() {
            if bomba_transform
                .translation
                .distance(dupix_transform.translation)
                < 50.
            {
                commands.entity(dupix).despawn_recursive();
                commands.entity(bomba).despawn_recursive();
            }
        }
    }
}

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bomba_vs_dupix);
    }
}
