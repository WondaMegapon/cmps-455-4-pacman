use crate::components::DestroyWhenNotThisState;
use crate::core::editor::EditorPlugin;
use crate::core::game::GamePlugin;
use crate::core::game_state::GameStatePlugin;
use crate::core::level::LevelPlugin;
use crate::core::prelude::*;
use bevy::prelude::*;
pub(super) struct CorePlugin; // The base of the game.

pub mod constants; // Cooonstants~
pub mod editor; // An eye on the editooor.
pub mod game; // For the game systems themselves.
pub mod game_state; // An eye on our states.
pub mod level; // For levely stuff.
pub mod prelude; // Yeah prelude!

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameStatePlugin, EditorPlugin, GamePlugin, LevelPlugin));
        app.add_systems(Startup, setup);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn clean_components(
    mut commands: Commands,
    discarded_items: Query<(Entity, &DestroyWhenNotThisState)>,
    current_game_state: Res<State<GameState>>,
) {
    // For each entity that isn't in the current Game State.
    for entity in discarded_items
        .iter()
        .filter(|item| &item.1 .0 != current_game_state.get())
    {
        commands.entity(entity.0).despawn(); // Getting rid of that thang.
    }
}
