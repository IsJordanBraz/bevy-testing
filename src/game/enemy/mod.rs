use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(Update, (
                enemy_movement,
                enemy_direction_on_collision,
                confine_enemy_movement,
                destroy_enemy,
                tick_enemy_spawn_timer,
                spawn_enemy_over_time,
            ).run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
        )
        .add_systems(OnExit(AppState::Game), destroy_all_enemies);
    }
}

