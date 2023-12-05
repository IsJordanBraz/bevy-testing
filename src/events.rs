use bevy::prelude::*;

#[derive(Event)]
pub struct ScoreCollision {
    pub score: u32,
}