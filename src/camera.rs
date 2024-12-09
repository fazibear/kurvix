use bevy::prelude::*;

pub struct CameraPlugin;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d {});
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
