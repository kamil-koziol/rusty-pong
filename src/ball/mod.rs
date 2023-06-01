use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn)
            .add_system(handle_movement);
    }
}

