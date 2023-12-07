use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::dynamics::RigidBody;
use crate::{events::*, AppState};

pub fn spawn_camera(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
            ..Default::default()
        }
    );
}

pub fn handle_score_collision_event(mut score_event: EventReader<ScoreCollision>) {
    for event in score_event.read() {
        println!("Your Score is {}", event.score);
    }    
}

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.get() != &AppState::Game {
            commands.insert_resource(NextState(Some(AppState::Game)));
            println!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if app_state.get() == &AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::Game)));
        } else if app_state.get() == &AppState::Game {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
        }
    }
}

pub fn spawn_ground(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>, 
    asset_server: Res<AssetServer>
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn((
        Collider::cuboid(window.width(), 20.0),
        RigidBody::Fixed,
    )).insert(Name::new("Ground"))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}
