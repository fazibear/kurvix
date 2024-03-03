mod animation;
mod camera;
mod cursor;
mod orzel;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            // Add the plugins
            DefaultPlugins,
            camera::CameraPlugin,
            cursor::CursorPlugin,
            orzel::OrzelPlugin,
            animation::AnimationPlugin,
        ))
        .run();
}
