mod animation;
mod background;
mod camera;
mod cleanup;
mod cursor;
mod orzel;
use bevy::prelude::*;

fn main() {
    let default_plugin = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Kurvix Saviours".to_string(),
            ..Default::default()
        }),
        ..default()
    });

    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins((
            // Add the plugins
            default_plugin,
            camera::CameraPlugin,
            cursor::CursorPlugin,
            animation::AnimationPlugin,
            cleanup::CameraPlugin,
            background::BackgroundPlugin,
            orzel::OrzelPlugin,
        ))
        .run();
}
