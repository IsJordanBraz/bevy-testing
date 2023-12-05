use bevy::prelude::*;

use crate::AppState;

use self::systems::{layout::{spawn_main_menu, despawn_main_menu}, interations::{interact_with_quit_button, interact_with_play_button}};

pub mod systems;
pub mod components;
pub mod styles;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), 
            spawn_main_menu
        )
        .add_systems(OnExit(AppState::MainMenu), 
            despawn_main_menu
        )
        .add_systems(Update, (interact_with_quit_button, interact_with_play_button.run_if(in_state(AppState::MainMenu))));
    }
}
