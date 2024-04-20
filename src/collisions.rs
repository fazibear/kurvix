use bevy::prelude::*;

use crate::animable::Animable;
use crate::bomba::Bomba;
use crate::dupix::Dupix;
use crate::info::Info;
use crate::movable::Movable;
use crate::orzel::Orzel;
use crate::GameState;

pub struct CollisionsPlugin;

fn is_colling(
    a_transform: &Transform,
    a_size: &Vec2,
    a_scale: f32,
    b_transform: &Transform,
    b_size: &Vec2,
    b_scale: f32,
) -> bool {
    let a_x = a_transform.translation.x;
    let b_x = b_transform.translation.x;
    let a_y = a_transform.translation.y;
    let b_y = b_transform.translation.y;

    let min_x = (a_size.x * a_scale / 2.) + (b_size.x * b_scale / 2.) * 0.8;
    let min_y = (a_size.y * a_scale / 2.) + (b_size.y * b_scale / 2.) * 0.8;

    let dist_x = (a_x - b_x).abs();
    let dist_y = (a_y - b_y).abs();

    dist_x < min_x && dist_y < min_y
}

fn bomba_vs_dupix(
    mut commands: Commands,
    mut info: Query<&mut Info>,
    q_bomba: Query<(Entity, &Transform), (With<Bomba>, Without<Dupix>)>,
    mut q_dupix: Query<
        (&mut Transform, &mut Movable, &mut Animable),
        (With<Dupix>, Without<Bomba>),
    >,
    assets: ResMut<AssetServer>,
) {
    let mut info = info.single_mut();
    for (bomba, bomba_transform) in q_bomba.iter() {
        for (mut dupix_transform, mut dupix_movable, mut dupix_animable) in q_dupix.iter_mut() {
            if bomba_transform
                .translation
                .distance(dupix_transform.translation)
                < 40.
            {
                commands.spawn(AudioBundle {
                    source: assets.load("bam.ogg"),
                    ..Default::default()
                });
                commands.entity(bomba).despawn_recursive();
                info.points += 1;
                dupix_movable.direction.y = -450.;
                dupix_transform.rotation.z = -0.3;
                dupix_animable.start_frame = 4;
                dupix_animable.end_frame = 5;
            }
        }
    }
}

fn orzel_vs_dupix(
    q_orzel: Query<&Transform, (With<Orzel>, Without<Dupix>)>,
    q_dupix: Query<&Transform, (With<Dupix>, Without<Orzel>)>,
    mut state: ResMut<NextState<GameState>>,
) {
    let orzel_transform = q_orzel.single();
    for dupix_transform in q_dupix.iter() {
        if is_colling(
            orzel_transform,
            &Vec2::new(780., 360.),
            0.2,
            dupix_transform,
            &Vec2::new(384., 114.),
            0.5,
        ) {
            state.set(GameState::Crashed);
        }
    }
}

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bomba_vs_dupix.run_if(in_state(GameState::Playing)));
        app.add_systems(Update, orzel_vs_dupix.run_if(in_state(GameState::Playing)));
    }
}
