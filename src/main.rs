use bevy::{
    prelude::*,
    window::{PrimaryWindow, VideoModeSelection, WindowMode},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_fullscreen)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default());
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
