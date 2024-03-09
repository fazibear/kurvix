use bevy::prelude::*;

pub struct AnimablePlugin;

#[derive(Component, Debug)]
pub struct Animable {
    pub current_frame: usize,
    pub passed_frames: f32,
    pub start_frame: usize,
    pub end_frame: usize,
    pub fps: f32,
}

fn increment_frame(animable: &mut Animable, times: usize) {
    let mut next_frame = animable.current_frame;
    for _ in 0..times {
        next_frame += 1;
        if next_frame > animable.end_frame {
            next_frame = animable.start_frame;
        }
    }
    animable.current_frame = next_frame;
}

fn animate(mut query: Query<&mut Animable>, time: Res<Time>) {
    for mut animable in query.iter_mut() {
        animable.passed_frames += time.delta_seconds() * animable.fps;
        let passed_whole_frames = animable.passed_frames.floor() as usize;
        increment_frame(&mut animable, passed_whole_frames);
        animable.passed_frames -= animable.passed_frames.floor();
    }
}

impl Plugin for AnimablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate);
    }
}
