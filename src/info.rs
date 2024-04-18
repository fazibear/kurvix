use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct InfoPlugin;

#[derive(Component, Debug)]
pub struct Info {
    pub points: i32,
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let text_style = TextStyle {
        font: assets.load("font.ttf"),
        font_size: 20.0,
        color: Color::rgb(0., 0.8, 0.),
    };

    commands.spawn((
        Info { points: 0 },
        Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "MIEJSCE:    GALAKTYKA    KURVIX\n".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "MISJA:    NAPIERDALAC\n".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "PUNKTY:    ".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: text_style.clone(),
                    },
                ],
                ..default()
            },
            transform: Transform::from_xyz(-640., 340., 0.),
            text_anchor: Anchor::TopLeft,
            ..default()
        },
    ));
}

fn update(mut query: Query<(&mut Info, &mut Text)>) {
    let (info, mut text) = query.single_mut();
    text.sections[3].value = format!("{}", info.points);
}

impl Plugin for InfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update);
    }
}
