use crate::components;
use crate::core::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

pub(super) struct EditorPlugin;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct EditorGizmos {} // Storing all of our fancy lil' editor gizmos.

#[derive(Resource)]
struct PossibleConnection(Option<usize>); // Allowing for connections to be wired if need be. (Junction, ConnectionPoint)

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<EditorGizmos>()
            .insert_resource(components::Junctions(Vec::new()))
            .insert_resource(components::Connections(Vec::new()))
            .insert_resource(PossibleConnection(None))
            .add_systems(Update, (render_editor).run_if(in_editor))
            .add_systems(
                FixedUpdate,
                (
                    manage_editors,
                    nodes_input.run_if(in_state(GameState::Editor(Editor::Nodes))),
                    connections_input.run_if(in_state(GameState::Editor(Editor::Connections))),
                )
                    .run_if(in_editor),
            );
    }
}

// Allows for swapping editors.
fn manage_editors(
    buttons: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if buttons.just_pressed(KeyCode::Tab) {
        match state.get() {
            GameState::Menu(_menu) => unimplemented!(),
            GameState::Editor(editor) => match editor {
                Editor::New => unimplemented!(),
                Editor::Nodes => next_state.set(GameState::Editor(Editor::Connections)),
                Editor::Connections => next_state.set(GameState::Editor(Editor::Nodes)),
                Editor::Saving => unimplemented!(),
            },
            GameState::Playing(_playing) => unimplemented!(),
        }
    }
}

// Editor input.
fn nodes_input(
    query_windows: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut junctions: ResMut<components::Junctions>,
    mut connections: ResMut<components::Connections>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = query_camera.single();

    // Handling LMB (Moving junctions.)
    if buttons.pressed(MouseButton::Left) {
        if let Some(cursor_position) = query_windows
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            if let Some(junction) = junctions.0.iter_mut().find(|junction| {
                (junction.0.x - cursor_position.x).powf(2.0)
                    + (junction.0.y - cursor_position.y).powf(2.0)
                    < EDITOR_JUNCTION_RADIUS.powf(2.0)
            }) {
                // If we found a junction under the cursor.
                // Move iiit.
                junction.0 = Vec2::new(cursor_position.x, cursor_position.y);
            }
        }
    }

    // Creating, deleting junctions. via LMB
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = query_windows
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            if junctions
                .0
                .iter()
                .find(|junction| {
                    (junction.0.x - cursor_position.x).powf(2.0)
                        + (junction.0.y - cursor_position.y).powf(2.0)
                        < EDITOR_JUNCTION_RADIUS.powf(2.0)
                })
                .is_none()
            {
                // Otherwise.
                // Creating our junction.
                junctions.0.push(components::Junction(
                    Vec2::new(cursor_position.x, cursor_position.y),
                    components::JunctionType::None,
                )); // Our new junction.
                for _i in 0..MAX_CONNECTIONS {
                    connections.0.push(None); // Creating new spaces.
                }
            }
        }
    }

    // Creating, deleting junctions. via RMB
    if buttons.just_pressed(MouseButton::Right) {
        if let Some(cursor_position) = query_windows
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            if let Some(junction) = junctions.0.iter().position(|junction| {
                (junction.0.x - cursor_position.x).powf(2.0)
                    + (junction.0.y - cursor_position.y).powf(2.0)
                    < EDITOR_JUNCTION_RADIUS.powf(2.0)
            }) {
                // If we found a junction under the cursor.
                for _i in 0..MAX_CONNECTIONS {
                    connections.0.remove(junction); // Removing this node's junctions.
                }
                connections.0.retain(|connection| match connection {
                    Some(x) => *x != junction,
                    None => true,
                }); // Removing other junctions.
                junctions.0.remove(junction); // Delebing it.
            }
        }
    }

    // Cycling type via MMB.
    if buttons.just_pressed(MouseButton::Middle) {
        if let Some(cursor_position) = query_windows
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            if let Some(junction) = junctions.0.iter_mut().find(|junction| {
                (junction.0.x - cursor_position.x).powf(2.0)
                    + (junction.0.y - cursor_position.y).powf(2.0)
                    < EDITOR_JUNCTION_RADIUS.powf(2.0)
            }) {
                // If we found a junction under the cursor.
                // Change that junction type.
                match junction.1 {
                    components::JunctionType::None => {
                        junction.1 = components::JunctionType::PowerPellet
                    }
                    components::JunctionType::PowerPellet => {
                        junction.1 = components::JunctionType::GhostHouse
                    }
                    components::JunctionType::GhostHouse => {
                        junction.1 = components::JunctionType::BonusItem
                    }
                    components::JunctionType::BonusItem => {
                        junction.1 = components::JunctionType::None
                    }
                }
            }
        }
    }
}

fn connections_input(
    mut commands: Commands,
    query_windows: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<ButtonInput<MouseButton>>,
    junctions: ResMut<components::Junctions>,
    mut connections: ResMut<components::Connections>,
    mut possible_connection: ResMut<PossibleConnection>,
) {
    // Same camera thing.
    let (camera, camera_transform) = query_camera.single();

    // Creating them connections.
    if buttons.just_pressed(MouseButton::Left) {
        // Finding our cursor.
        if let Some(cursor_position) = query_windows
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            // Finding a node.
            if let Some(junction) = junctions.0.iter().position(|junction| {
                (junction.0.x - cursor_position.x).powf(2.0)
                    + (junction.0.y - cursor_position.y).powf(2.0)
                    < (EDITOR_JUNCTION_RADIUS + EDITOR_JUNCTION_CONNECTION_POINT_RADIUS).powf(2.0)
            }) {
                // *Or* finding a viable slot to put the connection in.
                if let Some(connection_point) = (0..8).find(|connection_point| {
                    let junction_start_pos = Quat::mul_vec3(
                        Quat::from_rotation_z(
                            (360 / MAX_CONNECTIONS * connection_point) as f32
                                * std::f32::consts::PI
                                / 180.0,
                        ),
                        Vec3::new(EDITOR_JUNCTION_RADIUS, 0.0_f32, 0.0_f32),
                    )
                    .xy()
                        + junctions.0[junction].0;
                    (junction_start_pos.x - cursor_position.x).powf(2.0)
                        + (junction_start_pos.y - cursor_position.y).powf(2.0)
                        < EDITOR_JUNCTION_CONNECTION_POINT_RADIUS.powf(2.0)
                }) {
                    // We found the node!
                    // If there's nothing at this node.
                    if connections.0[junction * MAX_CONNECTIONS + connection_point].is_none() {
                        // Check to see if it has anything.
                        match possible_connection.0 {
                            Some(value) => {
                                // If it's not the same as our existing connection.
                                if value != junction * MAX_CONNECTIONS + connection_point {
                                    connections.0[value] =
                                        Some(junction * MAX_CONNECTIONS + connection_point); // Set the value.
                                    connections.0[junction * MAX_CONNECTIONS + connection_point] =
                                        Some(value); // Same the other way around.
                                    possible_connection.0 = None; // Reset our possible connections.
                                }
                            }
                            None => {
                                // We have no existing connection.
                                possible_connection.0 =
                                    Some(junction * MAX_CONNECTIONS + connection_point);
                                // Set the possible connection to the current one.
                            }
                        }
                    }
                }
            } else {
                // Oh, we didn't hit a node?
                possible_connection.0 = None; // Get rid of the selection, since they probably didn't want it.
            }
        }
    }

    // Deleting them connections.
    if buttons.just_pressed(MouseButton::Right) {
        possible_connection.0 = None; // Tossing the current selection, 'cause I'm assuming they didn't want it.

        // Finding our cursor.
        if let Some(cursor_position) = query_windows
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            // Finding a valid junction.
            if let Some(junction) = junctions.0.iter().position(|junction| {
                (junction.0.x - cursor_position.x).powf(2.0)
                    + (junction.0.y - cursor_position.y).powf(2.0)
                    < EDITOR_JUNCTION_RADIUS.powf(2.0)
            }) {
                // Junction found! Time to iterate.
                for our_connection in 0..MAX_CONNECTIONS {
                    // If we can find a valid connection.
                    if let Some(their_connection) =
                        connections.0[junction * MAX_CONNECTIONS + our_connection]
                    {
                        connections.0[their_connection] = None; // Get rid of the straggler.
                        connections.0[junction * MAX_CONNECTIONS + our_connection] = None;
                        // Get rid of us, too.
                    }
                }
            }
        }
    }
}

// Rendering the current state of the editor.
fn render_editor(
    mut editor_gizmos: Gizmos<EditorGizmos>,
    junctions: Res<components::Junctions>,
    connections: Res<components::Connections>,
    possible_connection: Res<PossibleConnection>,
) {
    // For each of the junctions we have.
    for (junction_index, junction) in junctions.0.iter().enumerate() {
        // For each of the eight possible connections this junction could have.
        for possible_connection_index in 0..MAX_CONNECTIONS {
            let junction_start_pos = Quat::mul_vec3(
                Quat::from_rotation_z(
                    (360 / MAX_CONNECTIONS * possible_connection_index) as f32
                        * std::f32::consts::PI
                        / 180.0,
                ),
                Vec3::new(EDITOR_JUNCTION_RADIUS, 0.0_f32, 0.0_f32),
            )
            .xy()
                + junction.0;
            // If an index exists.
            if let Some(found_junction_index) =
                connections.0[junction_index * MAX_CONNECTIONS + possible_connection_index]
            {
                println!(
                    "Connection at {:?} and {:?}",
                    junction_index * MAX_CONNECTIONS + possible_connection_index,
                    found_junction_index
                );
                // Get that position!
                let junction_end_pos = (Quat::mul_vec3(
                    Quat::from_rotation_z(
                        (360 / MAX_CONNECTIONS * (found_junction_index % MAX_CONNECTIONS)) as f32
                            * std::f32::consts::PI
                            / 180.0,
                    ),
                    Vec3::new(EDITOR_JUNCTION_RADIUS, 0.0_f32, 0.0_f32),
                )
                .xy()
                    + junctions.0[found_junction_index / MAX_CONNECTIONS].0
                    - junction_start_pos)
                    * 0.5;
                // Draw a line!
                editor_gizmos.ray_2d(
                    junction_start_pos,
                    junction_end_pos,
                    bevy::color::palettes::css::LAVENDER,
                );
                // And a happy green circle.
                editor_gizmos.circle_2d(
                    junction_start_pos,
                    EDITOR_JUNCTION_CONNECTION_POINT_RADIUS,
                    bevy::color::palettes::css::GREEN,
                );
            } else {
                editor_gizmos.circle_2d(
                    junction_start_pos,
                    EDITOR_JUNCTION_CONNECTION_POINT_RADIUS,
                    match possible_connection.0 {
                        Some(value) => {
                            if value
                                == (junction_index * MAX_CONNECTIONS + possible_connection_index)
                            {
                                bevy::color::palettes::css::RED
                            } else {
                                bevy::color::palettes::css::LAVENDER
                            }
                        }
                        None => bevy::color::palettes::css::GREY,
                    },
                );
            }
        }
        editor_gizmos.circle_2d(
            junction.0,
            EDITOR_JUNCTION_RADIUS,
            match junction.1 {
                components::JunctionType::None => bevy::color::palettes::css::LAVENDER,
                components::JunctionType::PowerPellet => bevy::color::palettes::css::GREEN,
                components::JunctionType::GhostHouse => bevy::color::palettes::css::BLUE,
                components::JunctionType::BonusItem => bevy::color::palettes::css::RED,
            },
        );
    }
}
