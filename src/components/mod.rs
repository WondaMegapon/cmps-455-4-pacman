use crate::core::prelude::GameState;
use bevy::prelude::*; // Always useful. // Other necessaries.

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum JunctionType {
    None,        // There's nothing special about this node. Put a pellet here.
    PowerPellet, // Hey, gotta get rid of those ghosts somehow.
    GhostHouse,  // Ghost will spawn from and return here.
    BonusItem,   // Bonus items will appear here.
}

#[derive(Component)]
pub struct DestroyWhenNotThisState(pub GameState); // Holds a given game state to destroy useless components on.
pub struct Junction(pub Vec2, pub JunctionType); // A position and a type, as it should be.
#[derive(Resource)]
pub struct Junctions(pub Vec<Junction>); // A container of junctions, for storing with levels.
#[derive(Resource)]
pub struct Connections(pub Vec<Option<usize>>); // Literally just a link between two junctions. The index is the first node, the usize is the second.
