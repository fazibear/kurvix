use bevy::input::mouse::MouseButtonInput;
use bevy::input::touch::TouchPhase;
use bevy::input::ButtonState;
use bevy::prelude::*;

use crate::GameState;
pub struct SoundsPlugin;

//#[derive(Component, Debug)]
//pub struct Sounds {}

fn napierdalamy(assets: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioPlayer::new(assets.load("napierdalamy.ogg")));
}

fn bum(assets: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioPlayer::new(assets.load("bum.ogg")));
}

fn dupix(assets: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioPlayer::new(assets.load("dupix.ogg")));
}

fn mouse_shoot(
    mut commands: Commands,
    mut events: EventReader<MouseButtonInput>,
    assets: Res<AssetServer>,
) {
    for event in events.read() {
        if event.state == ButtonState::Pressed {
            commands.spawn(AudioPlayer::new(assets.load("shot.ogg")));
        }
    }
}

fn touch_shoot(
    mut commands: Commands,
    mut events: EventReader<TouchInput>,
    assets: Res<AssetServer>,
) {
    for event in events.read() {
        if event.phase == TouchPhase::Ended {
            commands.spawn(AudioPlayer::new(assets.load("shot.ogg")));
        }
    }
}

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, dupix);
        app.add_systems(OnExit(GameState::Crashed), napierdalamy);
        app.add_systems(OnExit(GameState::Playing), bum);
        app.add_systems(Update, mouse_shoot.run_if(in_state(GameState::Playing)));
        app.add_systems(Update, touch_shoot.run_if(in_state(GameState::Playing)));
    }
}
