use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};

use crate::AppState;

use self::{systems::{layout::{spawn_main_menu, despawn_main_menu}, interations::{interact_with_quit_button, interact_with_play_button}}, styles::{setup, text_update_system}};

pub mod systems;
pub mod components;
pub mod styles;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin).add_systems(OnEnter(AppState::MainMenu), 
            spawn_main_menu
        )
        .add_systems(OnExit(AppState::MainMenu), 
            despawn_main_menu
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (
            text_update_system,
            interact_with_quit_button, 
            interact_with_play_button.run_if(in_state(AppState::MainMenu))));
    }
}
