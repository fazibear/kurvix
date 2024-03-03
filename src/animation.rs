use bevy::prelude::*;

pub struct AnimationPlugin;

#[derive(Component, Debug)]
pub struct Animation {
    pub current_frame: usize,
    pub frames: usize,
    pub passed_frames: f32,
    pub fps: f32,
}

fn animate(mut query: Query<&mut Animation>, time: Res<Time>) {
    for mut animation in query.iter_mut() {
        animation.passed_frames += time.delta_seconds() * animation.fps;
        let passed_whole_frames = animation.passed_frames.floor() as usize;
        if passed_whole_frames > 0 {
            animation.current_frame =
                (animation.current_frame + passed_whole_frames) % animation.frames;
            animation.passed_frames -= animation.passed_frames.floor();
        }
    }
}

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate);
    }
}
