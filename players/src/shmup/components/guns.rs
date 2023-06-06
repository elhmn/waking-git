use bevy::prelude::*;

/// Basic enemy gun
#[derive(Component, Debug)]
pub struct SimpleGun {
    pub cooldown_timer: Timer,
}

impl Default for SimpleGun {
    fn default() -> Self {
        Self {
            cooldown_timer: Timer::from_seconds(2., TimerMode::Once),
        }
    }
}

#[derive(Component, Debug)]
pub struct FastGun {
    pub cooldown_timer: Timer,
    pub count_fired_bullets: u8,
}

impl Default for FastGun {
    fn default() -> Self {
        Self {
            cooldown_timer: Timer::from_seconds(0.5, TimerMode::Once),
            count_fired_bullets: 0,
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    // Towards the player
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
    East,
    West,
}

#[derive(Component, Debug)]
pub struct MultiDirectionCircleGun {
    pub cooldown_timer: Timer,
    pub count_fired_bullets: u8,
    pub directions: Vec<Direction>,
}

impl Default for MultiDirectionCircleGun {
    fn default() -> Self {
        Self {
            cooldown_timer: Timer::from_seconds(2.5, TimerMode::Once),
            count_fired_bullets: 0,
            directions: vec![
                Direction::North,
                Direction::NorthEast,
                Direction::NorthWest,
                Direction::South,
                Direction::SouthEast,
                Direction::SouthWest,
                Direction::East,
                Direction::West,
            ],
        }
    }
}

#[derive(Component, Debug)]
pub struct MultiDirectionRectangleGun {
    pub cooldown_timer: Timer,
    pub count_fired_bullets: u8,
    pub directions: Vec<Direction>,
}

impl Default for MultiDirectionRectangleGun {
    fn default() -> Self {
        Self {
            cooldown_timer: Timer::from_seconds(2.5, TimerMode::Once),
            count_fired_bullets: 0,
            directions: vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ],
        }
    }
}
