use bevy::prelude::*;

/// Pattern1 defines a movement pattern
#[derive(Component, Debug)]
// We could define better names
pub struct Pattern1 {
    pub speed: f32,
}

/// Pattern2 defines a movement pattern
#[derive(Component, Debug)]
pub struct Pattern2 {
    pub speed: f32,
}

/// Pattern3 defines a movement pattern
#[derive(Component, Debug)]
pub struct Pattern3 {
    pub speed: f32,
}
