use crate::components::*;
use crate::core::prelude::*;
use bevy::prelude::*;

pub(super) struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing(Playing::New)), spawn_agents)
        .add_systems(OnExit(GameState::Playing(Playing::Running)), remove_agents)
        .add_systems(FixedUpdate, (move_agents).run_if(in_state(GameState::Playing(Playing::Running))));
    }
}

// Spawn in an agent.
fn spawn_agents(
    mut commands: Commands,
    junctions: Res<components::Junctions>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for junction in junctions.0 {
        match junction.1 {
            components::JunctionType::None => {},
            components::JunctionType::PowerPellet => {},
            components::JunctionType::GhostHouse => {
                commands.spawn((
                    meshes.add(Circle::new(EDITOR_JUNCTION_RADIUS / 2.0)),
                    materials.add(Color::hsl(0.0, 0.0, 0.0)),
                    Transform::from_xyz(junction.0.x, junction.0.y, 0.0),
                    components::NavAgent {
                        junction_target: None,
                        junction_pool: Vec::new(),
                        type_base: components::NavAgentType::Shadow,
                        type_modified: None,
                    }
                ));
            },
            components::JunctionType::BonusItem => {
                commands.spawn((
                    meshes.add(Circle::new(EDITOR_JUNCTION_RADIUS / 2.0)),
                    materials.add(Color::hsl(0.0, 0.0, 0.0)),
                    Transform::from_xyz(junction.0.x, junction.0.y, 0.0),
                    components::NavAgent {
                        junction_target: None,
                        junction_pool: Vec::new(),
                        type_base: components::NavAgentType::Player,
                        type_modified: None,
                    }
                ));
            },
        }
    }
}

// Get rid of them.
fn remove_agents(
    mut commands: Commands,
    discarded: Query<(Entity), With<NavAgent>>
) {
    for discarded_entity in discarded.iter() {
        commands.entity(discarded_entity).despawn(); // Byebye.
    }
}

// Have it move around the grid.
fn move_agents(
    time: Res<Time>,
    agents: Query<(&mut Transform, &mut NavAgent)>,
    junctions: Res<components::Junctions>,
    connections: Res<components::Connections>,
) {
    for (transform, nav_agent) in agents.iter() {
        if let Some(valid_position) = nav_agent.junction_target {
            // We have a valid position to go to.

            // If we're close to a node.
            if (junctions.0[valid_position].0.x - transform.translation.x).powf(2.0) < (2.0_f32).powf(2.0) {
                // Resetting the pool.
                nav_agent.junction_pool = Vec::new();

                // Making a new set of connections.
                for possible_target in 0..MAX_CONNECTIONS {
                    // If there is a valid target, add it to our target pool.
                    if let Some(found_target) = connections.0[valid_position * MAX_CONNECTIONS + possible_target] {
                        nav_agent.junction_pool.push(found_target / MAX_CONNECTIONS); // Converting it back into a junction.
                    }
                }

                // And if we successfully find a target.
                if let Some(valid_target) = nav_agent.junction_pool[0] {
                    nav_agent.junction_target = valid_target;
                }
            } else {
                // Just keep swimming.
                transform.translation += (transform.translation - Vec3::new(junctions.0[valid_position].0.x, junctions.0[valid_position].0.y, 0.0)).normalize() * time.delta_seconds();
            }
        }
    }
}