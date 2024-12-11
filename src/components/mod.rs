use crate::core::prelude::GameState;
use bevy::prelude::*; // Always useful. // Other necessaries.

// General Components.
#[derive(Component)]
pub struct DestroyWhenNotThisState(pub GameState); // Holds a given game state to destroy useless components on.

// These are for anything that needs to navigate the level mesh.
#[derive(Component)]
pub struct NavAgent {
    pub junction_target: Option<usize>, // This is the junction this agent is currently headed towards. (None for no movement)
    pub junction_pool: Vec<usize>, // These are all the available junctions this node can travel towards.
    pub type_base: NavAgentType,   // This is the behavior that the agent inherits.
    pub type_modified: Option<NavAgentType>, // This is the behavior that the agent can be granted by external means.
}
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum NavAgentType {
    Player,  // Pacman. Controller of their own destiny.
    Shadow,  // Blinky. Will always chase down the Player's exact position.
    Sneaky,  // Pinky. Will chase down the junction that the Player is currently targetting.
    Moody,   // Inky. Will chase down the position ahead of Shadow's current target position.
    Pokey, // Clyde. Will chase the Player until they're close enough, then they'll choose to run away.
    Fearful, // Fruit. Will run away from the Player.
}

// Level Components.
pub struct Junction(pub Vec2, pub JunctionType); // A position and a type, as it should be.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum JunctionType {
    None,        // There's nothing special about this node. Put a pellet here.
    PowerPellet, // Hey, gotta get rid of those ghosts somehow.
    GhostHouse,  // Ghost will spawn from and return here.
    BonusItem,   // Bonus items will appear here.
}
#[derive(Resource)]
pub struct Junctions(pub Vec<Junction>); // A container of junctions, for storing with levels.
#[derive(Resource)]
pub struct Connections(pub Vec<Option<usize>>); // Literally just a link between two junctions. The index is the first node, the usize is the second.
