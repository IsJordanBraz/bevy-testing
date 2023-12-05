use bevy::prelude::*;

use crate::{game::enemy::resources::*, AppState};

pub mod components;
mod systems;

use systems::*;

use super::SimulationState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(Update, (
                player_movement.before(confine_player_movement), 
                confine_player_movement,
            ).run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
            ).add_systems(OnExit(AppState::Game), destroy_player);
    }
}