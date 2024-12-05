use bevy::input::mouse::MouseButtonInput;
use bevy::input::touch::TouchPhase;
use bevy::input::ButtonState;
use bevy::prelude::*;

use crate::GameState;

pub struct CrashTextPlugin;

#[derive(Component, Debug)]
pub struct CrashText {}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        CrashText {},
        Text::new("PRZEGRALES\n\nWDEPNIJ COS\nI  NAPIERDALAJ"),
        Node {
            align_self: AlignSelf::Center,
            width: Val::Percent(100.),
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        TextFont {
            font: assets.load("font.ttf"),
            font_size: 60.0,
            ..default()
        },
        TextColor(Color::srgb(0., 0.8, 0.)),
    ));
}

fn show(mut query: Query<&mut Visibility, With<CrashText>>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn hide(mut query: Query<&mut Visibility, With<CrashText>>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
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

impl Plugin for CrashTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(OnEnter(GameState::Crashed), show);
        app.add_systems(OnExit(GameState::Crashed), hide);
        app.add_systems(Update, mouse_start.run_if(in_state(GameState::Crashed)));
        app.add_systems(Update, touch_start.run_if(in_state(GameState::Crashed)));
    }
}
