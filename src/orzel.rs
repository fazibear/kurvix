use bevy::prelude::*;

use crate::animation::Animation;

pub struct OrzelPlugin;

#[derive(Component, Debug, Default)]
pub struct Orzel {}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("orzel_7.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(780., 1440. / 4.),
        1,
        4,
        None,
        None,
    ));

    // Use only the subset of sprites in the sheet that make up the run animation
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Orzel {},
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas { layout, index: 0 },
            ..default()
        },
        Animation {
            passed_frames: 0.,
            current_frame: 0,
            frames: 4,
            fps: 15.,
        },
    ));
}

fn update(mut query: Query<(&Animation, &mut TextureAtlas), With<Orzel>>) {
    let (animation, mut texture_atlas) = query.single_mut();
    texture_atlas.index = animation.current_frame;
}

impl Plugin for OrzelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update);
    }
}
