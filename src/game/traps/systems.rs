use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::{geometry::Collider, dynamics::RigidBody};

pub fn add_trap_center(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>, 
    asset_server: Res<AssetServer>
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/trap1.png"),
            transform: Transform::from_xyz(window.width() / 2.0, 10.0, 0.),
            ..default()
        },
        Collider::ball(30.0),
        RigidBody::Fixed,
    ));
}

pub fn add_force_to_enemy(

) {

}