use bevy::prelude::*;

pub struct BackgroundPlugin;
use rand::Rng;

#[derive(Component, Debug, Default)]
pub struct Background {
    pub display_time: f32,
    pub displayed_time: f32,
}

#[derive(Component, Debug, Default)]
pub struct Stars;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut rng = rand::thread_rng();

    let stars = asset_server.load("stars.png");

    for y in -5..5 {
        for x in -5..5 {
            commands.spawn((
                Stars {},
                Sprite::from_image(stars.clone()),
                Transform::default()
                    .with_scale(Vec3::splat(0.5))
                    .with_translation(Vec3::new(
                        (x * 200 + rng.gen_range(300..500)) as f32,
                        (y * 200 + rng.gen_range(300..500)) as f32,
                        -0.9,
                    )),
            ));
        }
    }

    let texture = asset_server.load("background.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(912, 16),
        1,
        4,
        None,
        None,
    ));

    for _ in 0..10 {
        commands.spawn((
            Background {
                display_time: 0.1,
                displayed_time: 0.0,
            },
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: layout.clone(),
                    index: rng.gen_range(0..4),
                },
            ),
            Transform::default()
                .with_scale(Vec3::splat(0.6))
                .with_translation(Vec3::new(0.0, 0.0, -0.8)),
        ));
    }
}

fn update(
    mut query: Query<(&mut Background, &mut Sprite, &mut Transform), With<Background>>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    for (mut background, mut sprite, mut transform) in query.iter_mut() {
        if time.elapsed_secs() - background.displayed_time > background.display_time {
            background.displayed_time = time.elapsed_secs();
            transform.translation.x = rng.gen_range(-500.0..500.0);
            transform.translation.y = rng.gen_range(-400.0..400.0);
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = rng.gen_range(0..4);
            }
        } else {
            // transform.translation.x += 5.0
        }
    }
}

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update);
    }
}
