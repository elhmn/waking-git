use bevy::prelude::*;

pub const COLOR_DESTRUCTIBLE: &str = "fc6c12";
pub const COLOR_UNDESTRUCTIBLE: &str = "cc4dcc";

#[derive(Component, Debug)]
pub struct Bullet {
    pub life_timer: Timer,
    pub speed: f32,
    pub damage: f32,
    pub direction: Vec2,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            life_timer: Timer::from_seconds(5., TimerMode::Once),
            speed: 500.,
            damage: 1.,
            direction: Vec2::new(0., 0.),
        }
    }
}
