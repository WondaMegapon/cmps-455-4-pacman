use crate::components;
use crate::core::prelude::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};

pub(super) struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                manage_editors,
                (nodes_input).run_if(in_state(GameState::Editor(Editor::Nodes))),
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

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut nodes: Query<(Entity, &mut Transform, &mut components::Node)>,
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
            if let Some(node) = nodes.iter().find(|node| {
                (node.1.translation.x - cursor_position.x).powf(2.0)
                    + (node.1.translation.y - cursor_position.y).powf(2.0)
                    < 30.0_f32.powf(2.0)
            }) {
                // If we found a node under the cursor.
                // Delebing it.
                commands.entity(node.0).despawn();
            } else {
                // Otherwise.
                // Creating our node.
                commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Circle { radius: 30.0 })),
                        material: materials.add(Color::linear_rgb(0.27, 0.05, 0.5)),
                        transform: Transform::from_xyz(cursor_position.x, cursor_position.y, 0.0),
                        ..default()
                    })
                    .insert(components::Node(components::NodeType::None));
            }
        }
    }

    // Cycling type via RMB.
    if buttons.just_pressed(MouseButton::Right) {
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
