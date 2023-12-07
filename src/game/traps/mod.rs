use bevy::prelude::*;

use self::systems::add_trap_center;

mod systems;

pub struct TrapsPlugin;

impl Plugin for TrapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_trap_center);
    }
}