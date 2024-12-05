use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct CursorPlugin;

fn hide_cursor(
    mut query: Query<&mut Window, With<PrimaryWindow>>,
    mut events: EventReader<CursorEntered>,
) {
    let mut window = query.single_mut();
    for _event in events.read() {
        window.cursor_options.visible = false;
    }
}

fn show_cursor(
    mut query: Query<&mut Window, With<PrimaryWindow>>,
    mut events: EventReader<CursorLeft>,
) {
    let mut window = query.single_mut();
    for _event in events.read() {
        window.cursor_options.visible = true;
    }
}

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (hide_cursor, show_cursor));
    }
}
