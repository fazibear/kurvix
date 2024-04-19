use bevy::input::mouse::MouseButtonInput;
use bevy::input::touch::TouchPhase;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::GameState;

pub struct StartPlugin;

#[derive(Component, Debug)]
pub struct Start {}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let start_style = TextStyle {
        font: assets.load("font.ttf"),
        font_size: 80.0,
        color: Color::rgb(0., 0.8, 0.),
    };

    let start_sub_style = TextStyle {
        font: assets.load("font.ttf"),
        font_size: 40.0,
        color: Color::rgb(0., 0.8, 0.),
    };

    commands.spawn((
        Start {},
        Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection::new("PRZEGRALES\n", start_sub_style.clone()),
                    TextSection::new("WDEPNIJ  COS\n", start_style.clone()),
                    TextSection::new("I  NAPIERDALAJ", start_sub_style.clone()),
                ],
                justify: JustifyText::Center,
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
            text_anchor: Anchor::Center,
            ..default()
        },
    ));
}

fn show(mut query: Query<&mut Visibility, With<Start>>) {
    let mut visibility = query.single_mut();
    *visibility = Visibility::Visible;
}

fn hide(mut query: Query<&mut Visibility, With<Start>>) {
    let mut visibility = query.single_mut();
    *visibility = Visibility::Hidden;
}

fn mouse_start(mut events: EventReader<MouseButtonInput>, mut state: ResMut<NextState<GameState>>) {
    for event in events.read() {
        if event.state == ButtonState::Pressed {
            state.set(GameState::Playing);
        }
    }
}

fn touch_start(mut events: EventReader<TouchInput>, mut state: ResMut<NextState<GameState>>) {
    for event in events.read() {
        if event.phase == TouchPhase::Ended {
            state.set(GameState::Playing);
        }
    }
}

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(OnEnter(GameState::Crashed), show);
        app.add_systems(OnExit(GameState::Crashed), hide);
        app.add_systems(Update, mouse_start.run_if(in_state(GameState::Crashed)));
        app.add_systems(Update, touch_start.run_if(in_state(GameState::Crashed)));
    }
}
