use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Player {
    pub speed: f32,
    pub hp: f32,
}

#[derive(Component, Debug, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
