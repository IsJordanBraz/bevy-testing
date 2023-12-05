use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::enemy::components::*;
use crate::game::enemy::resources::*;
use crate::game::score::resources::*;
use crate::events::ScoreCollision;

pub const NUMBER_ENEMIES: usize = 1;
pub const ENEMY_SIZE: f32 = 60.0;
pub const ENEMY_SPEED: f32 = 500.0;

pub fn spawn_enemies(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>, 
    asset_server: Res<AssetServer>
) {
    let window: &Window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_ENEMIES {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/enemy.png"),
                transform: Transform::from_xyz(window.width() - 20.0 , 30.0, 0.),
                ..default()
            },
            Enemy {
                direction: Vec2 { x: (-10.0), y: (0.0) }.normalize()
            },
        ));
    }
}

pub fn destroy_all_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for enemy_entity in &enemy_query {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn enemy_direction_on_collision(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut score_event: EventWriter<ScoreCollision>
) {
    let window: &Window = window_query.get_single().unwrap();

    let half_enemy_size: f32 = ENEMY_SIZE / 2.0;

    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;

        let translation = transform.translation;

        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }

        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            score_event.send(ScoreCollision { score: 1 });
            let sound_effect = asset_server.load("sounds/footstep_concrete_000.ogg");
            commands.spawn(AudioBundle {
                source: sound_effect,             
                ..default()             
              });
        }
    }    
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();

    let half_enemy_size: f32 = ENEMY_SIZE / 2.0;

    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }  
}

pub fn destroy_enemy(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut score: ResMut<Score>,
) {
    for (enemy, transform) in &enemy_query {
        if transform.translation.x < 100.0 {
            score.value += 1;
            commands.entity(enemy).despawn();
        }
    } 
}

pub fn spawn_enemy_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window: &Window = window_query.get_single().unwrap();

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/enemy.png"),
                transform: Transform::from_xyz(window.width() - 20.0 , 30.0, 0.),
                ..default()
            },
            Enemy {
                direction: Vec2 { x: (-10.0), y: (0.0) }.normalize()
            },
        ));        
    }
}

pub fn tick_enemy_spawn_timer(mut spawn_time: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    spawn_time.timer.tick(time.delta());
}
