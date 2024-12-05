use bevy::prelude::*;

pub struct CleanupPlugin;

fn cleanup(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        if transform.translation().x > 700.0
            || transform.translation().x < -1000.0
            || transform.translation().y > 1000.0
            || transform.translation().y < -1000.0
        {
            commands.entity(entity).despawn_recursive();
        }
    }
}

impl Plugin for CleanupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cleanup);
    }
}
