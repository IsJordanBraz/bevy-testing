use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
    pub grounded: bool,
}
