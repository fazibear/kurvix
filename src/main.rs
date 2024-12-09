mod animable;
mod background;
mod bomba;
mod camera;
mod cleanup;
mod collisions;
mod crash_text;
mod cursor;
mod dupix;
mod info_text;
mod movable;
mod orzel;
mod sounds;

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
            resolution: (1280., 720.).into(),
            resizable: false,
            ..Default::default()
        }),
        ..default()
    });

    App::new()
        .add_plugins((
            // Add the plugins
            default_plugin,
            animable::AnimablePlugin,
            background::BackgroundPlugin,
            bomba::BombaPlugin,
            camera::CameraPlugin,
            cleanup::CleanupPlugin,
            collisions::CollisionsPlugin,
            crash_text::CrashTextPlugin,
            cursor::CursorPlugin,
            dupix::DupixPlugin,
            info_text::InfoTextPlugin,
            movable::MovablePlugin,
            orzel::OrzelPlugin,
            sounds::SoundsPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
        .insert_state(GameState::Playing)
        .insert_state(GameState::Crashed)
        .run();
}
