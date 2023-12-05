use bevy::prelude::*;

use crate::systems::handle_score_collision_event;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Update, (
                update_score, 
                handle_score_collision_event,
            ));
    }
}