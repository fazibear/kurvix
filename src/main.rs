mod animation;
mod camera;
mod orzel;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            // Add the plugins
            DefaultPlugins,
            //camera::CameraPlugin,
            orzel::OrzelPlugin,
            animation::AnimationPlugin,
        ))
        .run();
}
