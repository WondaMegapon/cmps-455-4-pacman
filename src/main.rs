use bevy::prelude::*; // The new sauce.

mod components;
mod core;

// The meat, the bones, the core of the program.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                position: WindowPosition::At((0, 0).into()),
                resolution: (1280.0, 720.0).into(),
                title: "PacMan".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(crate::core::CorePlugin)
        .run(); // Yay an app.
}
