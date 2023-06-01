use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn)
        .add_system(handle_movement)
        .add_system(handle_ball_collision)
        ;
    }
}
