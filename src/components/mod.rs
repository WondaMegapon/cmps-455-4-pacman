use bevy::prelude::*; // Always useful.

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum NodeType {
    None,        // There's nothing special about this node. Put a pellet here.
    PowerPellet, // Hey, gotta get rid of those ghosts somehow.
    GhostHouse,  // Ghost will spawn from and return here.
    BonusItem,   // Bonus items will appear here.
}

#[derive(Component)]
pub struct Node(pub NodeType); // A node type. The position will be handled by a seperate thingy.
pub struct Connection(pub usize, pub usize); // Literally just an index to the node.
