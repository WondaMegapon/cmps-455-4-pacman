use crate::components;
use crate::core::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

pub(super) struct EditorPlugin;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct EditorGizmos {}

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<EditorGizmos>()
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
    mut commands: Commands,
    query_windows: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut nodes: Query<(Entity, &mut Transform, &mut components::Node)>,
    mut connections: Query<(Entity, &components::Connection)>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = query_camera.single();

    // Handling LMB (Moving nodes.)
    if buttons.pressed(MouseButton::Left) {
        if let Some(cursor_position) = query_windows
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            if let Some(mut node) = nodes.iter_mut().find(|node| {
                (node.1.translation.x - cursor_position.x).powf(2.0)
                    + (node.1.translation.y - cursor_position.y).powf(2.0)
                    < 30.0_f32.powf(2.0)
            }) {
                // If we found a node under the cursor.
                // Move iiit.
                node.1.translation = Vec3::new(cursor_position.x, cursor_position.y, 0.0);
            }
        }
    }

    // Creating, deleting nodes. via LMB
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = query_windows
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            if nodes
                .iter()
                .find(|node| {
                    (node.1.translation.x - cursor_position.x).powf(2.0)
                        + (node.1.translation.y - cursor_position.y).powf(2.0)
                        < 30.0_f32.powf(2.0)
                })
                .is_none()
            {
                // Otherwise.
                // Creating our node.
                commands.spawn((
                    Transform::from_xyz(cursor_position.x, cursor_position.y, 0.0),
                    components::Node(components::NodeType::None),
                ));
            }
        }
    }

    // Creating, deleting nodes. via RMB
    if buttons.just_pressed(MouseButton::Right) {
        if let Some(cursor_position) = query_windows
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            if let Some(node) = nodes.iter().find(|node| {
                (node.1.translation.x - cursor_position.x).powf(2.0)
                    + (node.1.translation.y - cursor_position.y).powf(2.0)
                    < 30.0_f32.powf(2.0)
            }) {
                // If we found a node under the cursor.
                for connection in connections.iter_mut().filter(|connection| {
                    connection.1 .0 == node.0 || connection.1 .1 == Some(node.0)
                }) {
                    commands.entity(connection.0).despawn(); // Deleting it's connections.
                }
                commands.entity(node.0).despawn(); // Delebing it.
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
            if let Some(mut node) = nodes.iter_mut().find(|node| {
                (node.1.translation.x - cursor_position.x).powf(2.0)
                    + (node.1.translation.y - cursor_position.y).powf(2.0)
                    < 30.0_f32.powf(2.0)
            }) {
                // If we found a node under the cursor.
                // Change that node type.
                match node.2 .0 {
                    components::NodeType::None => node.2 .0 = components::NodeType::PowerPellet,
                    components::NodeType::PowerPellet => {
                        node.2 .0 = components::NodeType::GhostHouse
                    }
                    components::NodeType::GhostHouse => node.2 .0 = components::NodeType::BonusItem,
                    components::NodeType::BonusItem => node.2 .0 = components::NodeType::None,
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
    nodes: Query<(Entity, &mut Transform, &mut components::Node)>,
    mut connections: Query<(Entity, &mut components::Connection)>,
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
            if let Some(node) = nodes.iter().find(|node| {
                (node.1.translation.x - cursor_position.x).powf(2.0)
                    + (node.1.translation.y - cursor_position.y).powf(2.0)
                    < 30.0_f32.powf(2.0)
            }) {
                // Finding some poor malformed connection.
                if let Some(mut connection) = connections
                    .iter_mut()
                    .find(|connection| connection.1 .0 != node.0 && connection.1 .1.is_none())
                {
                    connection.1 .1 = Some(node.0); // Assigning that second connection
                } else {
                    commands.spawn(components::Connection(node.0, None)); // Assigning that first connection.
                }
            }
        }
    }

    // Deleting them connections.
    if buttons.just_pressed(MouseButton::Right) {
        // Finding our cursor.
        if let Some(cursor_position) = query_windows
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            // Finding a node.
            if let Some(node) = nodes.iter().find(|node| {
                (node.1.translation.x - cursor_position.x).powf(2.0)
                    + (node.1.translation.y - cursor_position.y).powf(2.0)
                    < 30.0_f32.powf(2.0)
            }) {
                // Finding some poor malformed connection.
                for connection in connections.iter_mut().filter(|connection| {
                    connection.1 .0 == node.0
                        || match connection.1 .1 {
                            Some(value) => value == node.0,
                            None => false,
                        }
                }) {
                    commands.entity(connection.0).despawn(); // Byye.
                }
            }
        } else {
            // Deleting all malformed connections.
            for connection in connections
                .iter()
                .filter(|connection| connection.1 .1.is_none())
            {
                commands.entity(connection.0).despawn(); // Begone.
            }
        }
    }
}

// Rendering the current state of the editor.
fn render_editor(
    mut editor_gizmos: Gizmos<EditorGizmos>,
    nodes: Query<(Entity, &Transform, &components::Node)>,
    connections: Query<(Entity, &components::Connection)>,
) {
    for node in nodes.iter() {
        editor_gizmos.circle_2d(
            node.1.translation.xy(),
            30.0,
            match node.2 .0 {
                components::NodeType::None => bevy::color::palettes::css::LAVENDER,
                components::NodeType::PowerPellet => bevy::color::palettes::css::GREEN,
                components::NodeType::GhostHouse => bevy::color::palettes::css::BLUE,
                components::NodeType::BonusItem => bevy::color::palettes::css::RED,
            },
        );
    }

    for connection in connections.iter() {
        if let Ok(point_a) = nodes.get(connection.1 .0) {
            if connection.1 .1.is_some() {
                if let Ok(point_b) = nodes.get(connection.1 .1.unwrap()) {
                    editor_gizmos.line_2d(
                        point_a.1.translation.xy(),
                        point_b.1.translation.xy(),
                        bevy::color::palettes::css::LAVENDER,
                    );
                }
            } else {
                editor_gizmos.circle_2d(
                    point_a.1.translation.xy(),
                    5.0,
                    bevy::color::palettes::css::RED,
                );
            }
        }
    }
}
