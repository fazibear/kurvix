use bevy::prelude::*;

pub struct CameraPlugin;

fn cleanup(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        if transform.translation().distance(Vec3::ZERO) > 1100.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cleanup);
    }
}
