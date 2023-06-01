use std::ptr::null;

use crate::ball::components::Ball;
use crate::ball::systems::BALL_SIZE;

use super::components::*;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy::sprite::collide_aabb::collide;

pub const SPEED: f32 = 500.0;
pub const PADDLE_HEIGHT: f32 = 128.0;
pub const PADDLE_WIDTH: f32 = 32.0;
pub const OFFSET: f32 = 50.0;

pub fn spawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let left_sprite = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1., 1., 1.),
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(OFFSET, window.height()/2., 0.)),
        ..default()
    };

    let mut right_sprite = left_sprite.clone();
    right_sprite.transform = Transform::from_translation(Vec3::new(window.width() -  OFFSET, window.height()/2., 0.));

    commands.spawn((
        left_sprite,
        Paddle { paddle_type: PaddleType::LEFT},
    ));

    commands.spawn((
        right_sprite,
        Paddle { paddle_type: PaddleType::RIGHT},
    ));
}

pub fn handle_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Paddle)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {

    let window = window_query.get_single().unwrap();

    for(mut transform, paddle) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        let up_key: KeyCode;
        let down_key: KeyCode;

        if paddle.paddle_type == PaddleType::LEFT {
            up_key = KeyCode::W;
            down_key = KeyCode::S;
        } else {
            up_key = KeyCode::Up;
            down_key = KeyCode::Down;
        }


        if keyboard_input.pressed(up_key) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(down_key) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        
        let move_vector = direction * SPEED * time.delta_seconds();

        transform.translation += move_vector;
        transform.translation = keep_in_bounds(transform.translation, &window);
    }
}

pub fn keep_in_bounds(mut vec: Vec3, window: &Window) -> Vec3 {
    if vec.y > window.height() - PADDLE_HEIGHT/2. {
        vec.y = window.height() - PADDLE_HEIGHT/2.;
    }

    if vec.y < PADDLE_HEIGHT/2. {
        vec.y = PADDLE_HEIGHT/2.;
    }
    return vec;
}

pub fn handle_ball_collision(
    mut query: Query<(&Transform, &Paddle)>,
    mut ball_query: Query<(&Transform, &mut Ball)>,
) {

    if let Ok((ball_transform, mut ball)) = ball_query.get_single_mut() {
        for (transform, paddle) in query.iter_mut() {
            if collide(transform.translation, Vec2 { x: PADDLE_WIDTH, y: PADDLE_HEIGHT }, ball_transform.translation, Vec2 {x: BALL_SIZE, y: BALL_SIZE }).is_some() {
                ball.move_dir.x *= -1.0;
            }
        }
    }
}
