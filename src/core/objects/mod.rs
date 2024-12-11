use crate::core::objects::agents::AgentPlugin;
use bevy::prelude::*;

// Do plugin stuff.
pub(super) struct ObjectPlugin;

pub mod agents;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AgentPlugin);
    }
}
