use bevy::prelude::*;

#[derive(PartialEq)]
pub enum PaddleType {
    LEFT,
    RIGHT
}

#[derive(Component)]
pub struct Paddle {
    pub paddle_type: PaddleType
}
