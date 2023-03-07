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

#[derive(Component, Debug)]
pub struct Gun {
    pub cooldown_timer: Timer,
}

impl Default for Gun {
    fn default() -> Self {
        Self {
            cooldown_timer: Timer::from_seconds(0.07, TimerMode::Once),
        }
    }
}
