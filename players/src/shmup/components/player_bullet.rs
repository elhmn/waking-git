use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Bullet {
    pub life_timer: Timer,
    pub speed: f32,
    pub damage: f32,
    pub direction: Vec2,
}

#[derive(Component, Debug, Default)]
pub struct BulletCollider {}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            life_timer: Timer::from_seconds(2., TimerMode::Once),
            speed: 2000.,
            damage: 1.,
            direction: Vec2::new(0., 0.),
        }
    }
}
