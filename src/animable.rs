use bevy::prelude::*;

pub struct AnimablePlugin;

#[derive(Component, Debug)]
pub struct Animable {
    pub current_frame: usize,
    pub frames: usize,
    pub passed_frames: f32,
    pub fps: f32,
}

fn animate(mut query: Query<&mut Animable>, time: Res<Time>) {
    for mut animable in query.iter_mut() {
        animable.passed_frames += time.delta_seconds() * animable.fps;
        let passed_whole_frames = animable.passed_frames.floor() as usize;
        if passed_whole_frames > 0 {
            animable.current_frame =
                (animable.current_frame + passed_whole_frames) % animable.frames;
            animable.passed_frames -= animable.passed_frames.floor();
        }
    }
}

impl Plugin for AnimablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate);
    }
}
