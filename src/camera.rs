use bevy::prelude::*;

pub struct CameraPlugin;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d {},
        OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::Fixed {
                width: 1280.,
                height: 720.,
            },
            ..OrthographicProjection::default_2d()
        },
    ));
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
