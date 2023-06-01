pub mod events;
pub mod paddle;
pub mod ball;

mod systems;

use ball::BallPlugin;
use bevy::prelude::*;
use events::*;
use systems::*;

use paddle::PaddlePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(BallPlugin)
        .add_plugin(PaddlePlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .run();
}
