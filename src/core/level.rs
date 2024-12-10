use crate::components;
use crate::core::prelude::*;
use bevy::prelude::*;

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(components::Junctions(Vec::new()))
            .insert_resource(components::Connections(Vec::new()))
            .add_systems(OnEnter(GameState::Playing(Playing::New)), level_build);
    }
}

fn level_build(junctions: ResMut<components::Junctions>) {
    // First load the level...
    // Uh... That's a todo.

    // Then spawn a graphic for each thingy.
}
