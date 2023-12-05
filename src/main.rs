use bevy::prelude::*;

pub mod events;
mod systems;

mod ui;
use ui::MainMenuPlugin;

mod game;
use game::GamePlugin;

use events::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()        
        .add_event::<ScoreCollision>()
        .add_plugins((GamePlugin, MainMenuPlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (
            exit_game,
            transition_to_game_state,
            transition_to_main_menu_state
        ))
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}
