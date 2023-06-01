use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;
use super::components::*;

pub const BALL_SPEED: f32 = 500.0;
pub const BALL_SIZE: f32 = 25.0;
pub const TWO_PI: f32 = PI * 2.0;

pub fn spawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    
    let mut rng = rand::thread_rng();
    let mut angle: f32= rng.gen();
    angle *= TWO_PI;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 1., 1.),
                custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(window.width()/2., window.height()/2., 0.)),
            ..default()
        },
        Ball { move_dir: Vec2::from_angle(angle)}
    ));

}

pub fn handle_movement(
    mut query: Query<(&mut Transform, &mut Ball)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut ball)) = query.get_single_mut() {
        let md = ball.move_dir * BALL_SPEED * time.delta_seconds();
        let dir = Vec3::new(md.x, md.y, 0.0);
        transform.translation +=  dir;

        let window = window_query.get_single().unwrap();
        handle_walls(&mut ball, transform.translation, &window);
    }
}

pub fn handle_walls(ball: &mut Ball, mut vec: Vec3, window: &Window) {
    if vec.y > window.height() - BALL_SIZE/2. {
        vec.y = window.height() - BALL_SIZE/2.;
        ball.move_dir.y *= -1.0;
    }

    if vec.y < BALL_SIZE/2. {
        vec.y = BALL_SIZE/2.;
        ball.move_dir.y *= -1.0;
    }
}
