use bevy::prelude::*;

use crate::animable::Animable;
pub struct OrzelPlugin;

#[derive(Component, Debug, Default)]
pub struct Orzel {}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("orzel.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(780., 360.),
        1,
        4,
        None,
        None,
    ));

    // Use only the subset of sprites in the sheet that make up the run animable
    commands.spawn((
        Orzel {},
        SpriteSheetBundle {
            texture,
            transform: Transform::from_scale(Vec3::splat(0.2)),
            atlas: TextureAtlas { layout, index: 0 },
            ..default()
        },
        Animable {
            passed_frames: 0.,
            current_frame: 0,
            start_frame: 0,
            end_frame: 3,
            fps: 15.,
        },
    ));
}

fn animate(mut query: Query<(&Animable, &mut TextureAtlas), With<Orzel>>) {
    for (animable, mut texture_atlas) in query.iter_mut() {
        texture_atlas.index = animable.current_frame;
    }
}

fn moves(
    mut events: EventReader<CursorMoved>,
    mut query: Query<&mut Transform, With<Orzel>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let mut orzel = query.single_mut();
    let (camera, camera_transform) = camera.single();

    for event in events.read() {
        let position = camera
            .viewport_to_world_2d(camera_transform, event.position)
            .unwrap();
        orzel.translation.x = position.x;
        orzel.translation.y = position.y;
    }
}

impl Plugin for OrzelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, animate);
        app.add_systems(Update, moves);
    }
}
