mod animable;
mod background;
mod bomba;
mod camera;
mod cleanup;
mod cursor;
mod dupix;
mod movable;
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
            animable::AnimablePlugin,
            movable::MovablePlugin,
            cleanup::CameraPlugin,
            background::BackgroundPlugin,
            bomba::BombaPlugin,
            orzel::OrzelPlugin,
            dupix::DupixPlugin,
        ))
        .run();
}
