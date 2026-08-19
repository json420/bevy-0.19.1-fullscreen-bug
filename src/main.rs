use bevy::{
    prelude::*,
    window::{
        PresentMode, PrimaryWindow, VideoModeSelection, WindowMode, WindowPlugin, WindowResized,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, toggle_fullscreen)
        .run();
}

fn toggle_fullscreen(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keyboard.just_pressed(KeyCode::F11) {
        if let Ok(mut window) = query.single_mut() {
            window.mode = match window.mode {
                WindowMode::Windowed => {
                    WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current)
                }
                _ => WindowMode::Windowed,
            };
        }
    }
}
