use bevy::prelude::*;

pub(super) struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::Editor(Editor::Nodes))
            .add_systems(FixedUpdate, transition_states); // Setting our default state.
    }
}

// TODO: Put a transition function here.

fn transition_states(
    buttons: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // match current_state.get() {
    //     Menu(menu) => todo!(),
    //     Editor(editor) => todo!(),
    //     Playing(playing) => todo!(),
    // }
    if buttons.just_pressed(KeyCode::Digit1) {
        next_state.set(GameState::Editor(Editor::Nodes));
    }

    if buttons.just_pressed(KeyCode::Digit2) {
        next_state.set(GameState::Playing(Playing::New));
    }

    if buttons.just_pressed(KeyCode::Digit3) {
        next_state.set(GameState::Playing(Playing::Running));
    }
}

#[derive(States, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    Menu(Menu),       // For menuy and UIy bits.
    Editor(Editor),   // For the editor and level creation.
    Playing(Playing), // For playing! The game and stuff.
}

#[derive(Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Menu {
    Title,    // The tiiiitle screen.
    Settings, // For the settings menu.
}

#[derive(Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Editor {
    New,         // Getting a new level up and running.
    Nodes,       // For adding/removing nodes and setting their types.
    Connections, // For connecting nodes together.
    Saving,      // For saving the nodes before quitting back to the main game loop.
}

#[derive(Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Playing {
    New,      // Getting a new level up and running and spawned and stuff.
    Ready,    // Allowing the player to see the level before it starts.
    Running,  // GO PACMAN GO!
    GameOver, // Stopping the game then and theeere.
    Complete, // The player did it and beat the level.
}

// pub fn in_menu(current_state: Res<State<GameState>>) -> bool {
//     matches!(
//         current_state.get(),
//         crate::core::prelude::GameState::Menu(_)
//     )
// }

pub fn in_editor(current_state: Res<State<GameState>>) -> bool {
    matches!(
        current_state.get(),
        crate::core::prelude::GameState::Editor(_)
    )
}

// pub fn in_playing(current_state: Res<State<GameState>>) -> bool {
//     matches!(
//         current_state.get(),
//         crate::core::prelude::GameState::Playing(_)
//     )
// }
