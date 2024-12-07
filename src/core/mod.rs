use crate::core::editor::EditorPlugin;
use crate::core::game_state::GameStatePlugin;
use bevy::prelude::*;
use game::GamePlugin;
pub(super) struct CorePlugin; // The base of the game.

pub mod constants; // Cooonstants~
pub mod editor; // An eye on the editooor.
pub mod game; // For the game systems themselves.
pub mod game_state; // An eye on our states.
pub mod prelude; // Yeah prelude!

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameStatePlugin, EditorPlugin, GamePlugin));
        app.add_systems(Startup, setup);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
