use bevy::{prelude::*, window::PrimaryWindow, app::AppExit};

const PLAYER_SIZE: f32 = 60.0;
const PLAYERS_SPEED: f32 = 500.0;

const NUMBER_ENEMIES: usize = 1;
const ENEMY_SIZE: f32 = 60.0;
const ENEMY_SPEED: f32 = 500.0;
const ENEMY_SPAWN_TIME: f32 = 2.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<ScoreCollision>()
        .add_systems(Startup, (spawn_player, spawn_camera, spawn_enemies))
        .add_systems(Update, (
            player_movement, 
            enemy_movement, 
            confine_player_movement,
            enemy_direction_on_collision,
            confine_enemy_movement,
            destroy_enemy,
            update_score,
            tick_enemy_spawn_timer,
            spawn_enemy_over_time,
            exit_game,
            handle_score_collision_event
        ))
        .run();
}

#[derive(Component)]
struct Player {}

#[derive(Component)]
struct Enemy {
    direction: Vec2
}

#[derive(Resource)]
struct Score {
    value: u32,
}

#[derive(Resource)]
struct EnemySpawnTimer {
    timer: Timer,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer { timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating) }
    }
}

#[derive(Event)]
struct ScoreCollision {
    score: u32,
}

fn spawn_enemies(
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

fn spawn_player(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/enemy.png"),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
            ..default()
        },
        Player {},
    ));
}

fn spawn_camera(
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

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYERS_SPEED * time.delta_seconds();
    }
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window: &Window = window_query.get_single().unwrap();
        let half_player_size: f32 = PLAYER_SIZE / 2.0;
        let x_min: f32 = 0.0 + half_player_size;
        let x_max: f32 = window.width() - half_player_size;
        let y_min: f32 = 0.0 + half_player_size;
        let y_max: f32 = window.height() - half_player_size;

        let mut translation = player_transform.translation;

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

        player_transform.translation = translation;
    }
}

fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

fn enemy_direction_on_collision(
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

fn confine_enemy_movement(
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

fn destroy_enemy(
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

fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

fn tick_enemy_spawn_timer(mut spawn_time: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    spawn_time.timer.tick(time.delta());
}

fn spawn_enemy_over_time(
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

fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

fn handle_score_collision_event(mut score_event: EventReader<ScoreCollision>) {
    for event in score_event.read() {
        println!("Your Score is {}", event.score);
    }    
}