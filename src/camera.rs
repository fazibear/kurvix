use bevy::prelude::*;

pub struct CameraPlugin;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(-10., 0., 10.)),
        projection: OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::Fixed {
                width: 1280.,
                height: 720.,
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
