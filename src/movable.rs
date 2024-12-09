use bevy::prelude::*;

pub struct MovablePlugin;

#[derive(Component, Debug)]
pub struct Movable {
    pub direction: Vec2,
}

fn moveit(mut query: Query<(&mut Movable, &mut Transform)>, time: Res<Time>) {
    for (movable, mut transform) in query.iter_mut() {
        transform.translation.x += movable.direction.x * time.delta_secs();
        transform.translation.y += movable.direction.y * time.delta_secs();
    }
}

impl Plugin for MovablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, moveit);
    }
}
