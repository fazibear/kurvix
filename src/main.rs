mod animable;
mod background;
mod bomba;
mod camera;
mod cleanup;
mod collisions;
mod cursor;
mod dupix;
mod info;
mod movable;
mod orzel;
mod start;

use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Crashed,
    Playing,
}

fn main() {
    let default_plugin = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Kurvix".to_string(),
            resizable: false,
            ..Default::default()
        }),
        ..default()
    });

    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_state(GameState::Playing)
        .insert_state(GameState::Crashed)
        .add_plugins((
            // Add the plugins
            default_plugin,
            animable::AnimablePlugin,
            background::BackgroundPlugin,
            bomba::BombaPlugin,
            camera::CameraPlugin,
            cleanup::CameraPlugin,
            collisions::CollisionsPlugin,
            cursor::CursorPlugin,
            dupix::DupixPlugin,
            movable::MovablePlugin,
            orzel::OrzelPlugin,
            info::InfoPlugin,
            start::StartPlugin,
        ))
        .run();
}
