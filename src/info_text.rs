use bevy::prelude::*;

use crate::GameState;

pub struct InfoTextPlugin;

#[derive(Component, Debug)]
pub struct InfoText {
    pub points: i32,
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        InfoText { points: 0 },
        Text::new(""),
        TextFont {
            font: assets.load("font.ttf"),
            font_size: 25.0,
            ..default()
        },
        TextColor(Color::srgb(0., 0.8, 0.)),
    ));
}

fn update(mut query: Query<(&mut InfoText, &mut Text)>) {
    for (info, mut text) in query.iter_mut() {
        *text = Text::new(format!(
        "MIEJSCE:     GALAKTYKA      KURVIX\nMISJA:             NAPIERDALAC\nPUNKTY:         {}",
        info.points
    ));
    }
}

fn reset(mut query: Query<&mut InfoText>) {
    for mut info in query.iter_mut() {
        info.points = 0;
    }
}

impl Plugin for InfoTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update);
        app.add_systems(OnEnter(GameState::Playing), reset);
    }
}
