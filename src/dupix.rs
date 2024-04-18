use bevy::prelude::*;

use crate::animable::Animable;
use crate::movable::Movable;
pub struct DupixPlugin;
use rand::Rng;

#[derive(Component, Debug, Default)]
pub struct Dupix {}

#[derive(Resource, Debug, Default)]
pub struct DupixAsset {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

fn spawn(assets: Res<DupixAsset>, mut commands: Commands, query: Query<&Dupix>) {
    let mut rng = rand::thread_rng();

    if query.iter().count() < 10 {
        commands.spawn((
            Dupix {},
            SpriteSheetBundle {
                texture: assets.texture.clone(),
                transform: Transform::default()
                    .with_scale(Vec3::splat(0.5))
                    .with_translation(Vec3::new(
                        (-800 - rng.gen_range(0..3000)) as f32,
                        rng.gen_range(-350..350) as f32,
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
                start_frame: 0,
                end_frame: 3,
                fps: 15.,
            },
            Movable {
                direction: Vec2::new(rng.gen_range(150..300) as f32, 0.),
            },
        ));
    }
}

fn animate(mut query: Query<(&Animable, &mut TextureAtlas), With<Dupix>>) {
    for (animable, mut texture_atlas) in query.iter_mut() {
        texture_atlas.index = animable.current_frame;
    }
}

fn load_asset(
    asset_server: Res<AssetServer>,
    mut dupix_asset: ResMut<DupixAsset>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    dupix_asset.texture = asset_server.load("dupix.png");
    dupix_asset.layout = layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(384., 114.),
        1,
        6,
        None,
        None,
    ));
}

impl Plugin for DupixPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DupixAsset>();
        app.add_systems(Startup, load_asset);
        app.add_systems(Update, animate);
        app.add_systems(Update, spawn);
    }
}
