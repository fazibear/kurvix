use bevy::prelude::*;

use crate::animation::Animation;
pub struct BombaPlugin;

#[derive(Component, Debug, Default)]
pub struct Bomba {}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("bomba.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(273., 208.),
        1,
        6,
        None,
        None,
    ));

    // Use only the subset of sprites in the sheet that make up the run animation
    commands.spawn((
        Bomba {},
        SpriteSheetBundle {
            texture,
            transform: Transform::from_scale(Vec3::splat(0.1)),
            atlas: TextureAtlas { layout, index: 0 },
            ..default()
        },
        Animation {
            passed_frames: 0.,
            current_frame: 0,
            frames: 6,
            fps: 15.,
        },
    ));
}

fn update(mut query: Query<(&Animation, &mut TextureAtlas), With<Bomba>>) {
    let (animation, mut texture_atlas) = query.single_mut();
    texture_atlas.index = animation.current_frame;
}

impl Plugin for BombaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update);
    }
}
