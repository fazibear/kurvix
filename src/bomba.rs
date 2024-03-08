use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

use crate::animable::Animable;
use crate::movable::Movable;
pub struct BombaPlugin;

#[derive(Component, Debug, Default)]
pub struct Bomba {}

#[derive(Resource, Debug, Default)]
pub struct BombaAsset {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

fn shoot(
    mut commands: Commands,
    mut events: EventReader<MouseButtonInput>,
    assets: Res<BombaAsset>,
    query: Query<&Transform, With<crate::orzel::Orzel>>,
) {
    use bevy::input::ButtonState;

    let orzel = query.single();

    for event in events.read() {
        if event.state == ButtonState::Pressed {
            commands.spawn((
                Bomba {},
                SpriteSheetBundle {
                    texture: assets.texture.clone(),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(0.1))
                        .with_translation(Vec3::new(
                            orzel.translation.x,
                            orzel.translation.y,
                            -0.5,
                        )),
                    atlas: TextureAtlas {
                        layout: assets.layout.clone(),
                        index: 0,
                    },
                    ..default()
                },
                Animable {
                    passed_frames: 0.,
                    current_frame: 0,
                    frames: 6,
                    fps: 15.,
                },
                Movable {
                    direction: Vec2::new(-500., 0.),
                },
            ));
        }
    }
}

fn animate(mut query: Query<(&Animable, &mut TextureAtlas), With<Bomba>>) {
    for (animable, mut texture_atlas) in query.iter_mut() {
        texture_atlas.index = animable.current_frame;
    }
}

fn load_asset(
    asset_server: Res<AssetServer>,
    mut bomba_asset: ResMut<BombaAsset>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    bomba_asset.texture = asset_server.load("bomba.png");
    bomba_asset.layout = layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(273., 208.),
        1,
        6,
        None,
        None,
    ));
}

impl Plugin for BombaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BombaAsset>();
        app.add_systems(Startup, load_asset);
        app.add_systems(Update, animate);
        app.add_systems(Update, shoot);
    }
}
