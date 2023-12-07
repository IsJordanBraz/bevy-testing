use bevy::prelude::*;

mod enemy;
mod player;
mod score;
mod systems;
mod traps;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use systems::toggle_simulation;

use crate::AppState;

use self::traps::TrapsPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_plugins((
                EnemyPlugin,
                PlayerPlugin, 
                ScorePlugin,
                TrapsPlugin
            ))
            .add_systems(Update, 
                toggle_simulation.run_if(in_state(AppState::Game))
            );
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Paused,
    Running,
}