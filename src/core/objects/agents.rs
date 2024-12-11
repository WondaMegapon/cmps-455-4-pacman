use crate::components::*;
use crate::core::prelude::*;
use bevy::prelude::*;

pub(super) struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing(Playing::New)), spawn_agents)
            .add_systems(OnExit(GameState::Playing(Playing::Running)), remove_agents)
            .add_systems(
                FixedUpdate,
                (move_agents).run_if(in_state(GameState::Playing(Playing::Running))),
            );
    }
}

// Spawn in an agent.
fn spawn_agents(
    mut commands: Commands,
    junctions: Res<Junctions>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (index, junction) in junctions.0.iter().enumerate() {
        match junction.1 {
            JunctionType::None => {}
            JunctionType::PowerPellet => {}
            JunctionType::GhostHouse => {
                for iterator in 0..4 {
                    commands.spawn((
                        bevy::sprite::MaterialMesh2dBundle {
                            mesh: meshes.add(Circle::new(EDITOR_JUNCTION_RADIUS * 0.9)).into(),
                            material: materials.add(Color::hsl(
                                match iterator {
                                    0 => 0.0,
                                    1 => 294.0,
                                    2 => 187.0,
                                    3 => 35.0,
                                    _ => 156.0,
                                },
                                1.0,
                                0.5,
                            )),
                            transform: Transform::from_xyz(junction.0.x, junction.0.y, 0.0),
                            ..default()
                        },
                        NavAgent {
                            junction_target: Some(index),
                            junction_pool: Vec::new(),
                            type_base: match iterator {
                                0 => NavAgentType::Shadow,
                                1 => NavAgentType::Sneaky,
                                2 => NavAgentType::Moody,
                                3 => NavAgentType::Pokey,
                                _ => NavAgentType::Fearful,
                            },
                            type_modified: None,
                        },
                    ));
                }
            }
            JunctionType::BonusItem => {
                commands.spawn((
                    bevy::sprite::MaterialMesh2dBundle {
                        mesh: meshes.add(Circle::new(EDITOR_JUNCTION_RADIUS * 0.9)).into(),
                        material: materials.add(Color::hsl(57.0, 1.0, 0.5)),
                        transform: Transform::from_xyz(junction.0.x, junction.0.y, 0.0),
                        ..default()
                    },
                    NavAgent {
                        junction_target: Some(index),
                        junction_pool: Vec::new(),
                        type_base: NavAgentType::Player,
                        type_modified: None,
                    },
                ));
            }
        }
    }
}

// Get rid of them.
fn remove_agents(mut commands: Commands, discarded: Query<Entity, With<NavAgent>>) {
    for discarded_entity in discarded.iter() {
        commands.entity(discarded_entity).despawn(); // Byebye.
    }
}

// Have it move around the grid.
fn move_agents(
    time: Res<Time>,
    buttons: Res<ButtonInput<KeyCode>>,
    mut agents: Query<(&mut Transform, &mut NavAgent)>,
    junctions: Res<Junctions>,
    connections: Res<Connections>,
) {
    for (mut transform, mut nav_agent) in &mut agents {
        if let Some(valid_position) = nav_agent.junction_target {
            // We have a valid position to go to.

            // If we're close to a node.
            if (junctions.0[valid_position].0.x - transform.translation.x).powf(2.0)
                + (junctions.0[valid_position].0.y - transform.translation.y).powf(2.0)
                < (2.0_f32).powf(5.0)
            {
                // Resetting the pool.
                nav_agent.junction_pool = Vec::new();

                // Making a new set of connections.
                for possible_target in 0..MAX_CONNECTIONS {
                    // If there is a valid target, add it to our target pool.
                    if let Some(found_target) =
                        connections.0[valid_position * MAX_CONNECTIONS + possible_target]
                    {
                        nav_agent.junction_pool.push(found_target / MAX_CONNECTIONS);
                        // Converting it back into a junction.
                    }
                }

                // If we're not the player, we can't go backwards... As long as there's options.
                if nav_agent.type_base != NavAgentType::Player && nav_agent.junction_pool.len() > 1
                {
                    if let Some(target) = nav_agent.junction_target {
                        nav_agent.junction_pool.retain(|junc| junc != &target);
                    }
                }

                // And to find a target.
                nav_agent.junction_target = match nav_agent.type_base {
                    // If we're the player.
                    NavAgentType::Player => {
                        // Getting the direction vector.
                        let direction = Vec3::new(
                            buttons.pressed(KeyCode::KeyD) as i32 as f32
                                - buttons.pressed(KeyCode::KeyA) as i32 as f32,
                            buttons.pressed(KeyCode::KeyW) as i32 as f32
                                - buttons.pressed(KeyCode::KeyS) as i32 as f32,
                            0.0,
                        );

                        // Turning that into a position relative to where the player currently is.
                        let target_transform = transform.transform_point(direction) * 2000.0;

                        if direction.distance(Vec3::ZERO).abs() < 0.25 {
                            nav_agent.junction_target // Just stay put if there's no input.
                        } else {
                            match nav_agent.junction_pool.iter().min_by(|x, y| {
                                target_transform
                                    .distance(Vec3::new(
                                        junctions.0[**x].0.x,
                                        junctions.0[**x].0.x,
                                        0.0,
                                    ))
                                    .total_cmp(&target_transform.distance(Vec3::new(
                                        junctions.0[**y].0.x,
                                        junctions.0[**y].0.x,
                                        0.0,
                                    )))
                            }) {
                                Some(value) => Some(*value),
                                None => None,
                            }
                        }
                    }
                    _ => Some(
                        nav_agent.junction_pool
                            [rand::random::<usize>() % nav_agent.junction_pool.len()],
                    ),
                    // NavAgentType::Shadow => {
                    //     match nav_agent.junction_pool.iter().min_by(|x, y| {
                    //         transform
                    //             .translation
                    //             .distance(Vec3::new(
                    //                 junctions.0[**x].0.x,
                    //                 junctions.0[**x].0.x,
                    //                 0.0,
                    //             ))
                    //             .total_cmp(&transform.translation.distance(Vec3::new(
                    //                 junctions.0[**y].0.x,
                    //                 junctions.0[**y].0.x,
                    //                 0.0,
                    //             )))
                    //     }) {
                    //         Some(value) => Some(*value),
                    //         None => None,
                    //     }
                    // }
                    // NavAgentType::Sneaky => todo!(),
                    // NavAgentType::Moody => todo!(),
                    // NavAgentType::Pokey => todo!(),
                    // NavAgentType::Fearful => todo!(),
                };
            } else {
                // Just keep swimming.
                transform.translation = transform.translation.move_towards(
                    Vec3::new(
                        junctions.0[valid_position].0.x,
                        junctions.0[valid_position].0.y,
                        0.0,
                    ),
                    time.delta_seconds() * 250.0,
                );
            }
        }
    }
}
