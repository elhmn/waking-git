use bevy::prelude::*;
use rand::prelude::*;

/// Pattern1 defines a movement pattern
#[derive(Component, Debug)]
// We could define better names
pub struct Pattern1 {
    pub speed: f32,
}

/// Pattern2 defines a movement pattern
#[derive(Component, Debug, Default)]
pub struct Pattern2 {
    pub speed: f32,
}

/// MoveTowards
#[derive(Component, Debug)]
pub struct MoveTowards {
    pub max_prediction_time: f32,

    pub initial_speed: f32,
    pub max_speed: f32,
    pub speed: f32,
    //The angular speed
    pub rotation: f32,
}

impl Default for MoveTowards {
    fn default() -> Self {
        let initial_speed = 25.;
        let max_speed = 50.;
        Self {
            max_prediction_time: 1000.,
            initial_speed,
            max_speed,
            speed: initial_speed,
            rotation: 0.1,
        }
    }
}

/// Pattern3 defines a movement pattern
#[derive(Component, Debug)]
pub struct Pattern3 {
    pub speed: f32,
    /// Indicate when should change direction
    pub timer: Timer,
    pub dir_x: f32,
    pub dir_y: f32,
}

pub fn pattern3_random_dir(p: &mut Pattern3) {
    let mut r = thread_rng();
    let prob = r.gen_range(0..100);

    if (20..40).contains(&prob) {
        p.dir_x = 1.;
        p.dir_y = 1.;
    } else if (40..60).contains(&prob) {
        p.dir_x = -1.;
        p.dir_y = -1.;
    } else if (60..80).contains(&prob) {
        p.dir_x = 1.;
        p.dir_y = -1.;
    } else if (80..100).contains(&prob) {
        p.dir_x = -1.;
        p.dir_y = 1.;
    }
}

pub fn pattern3_random_time(p: &mut Pattern3) {
    let mut r = thread_rng();
    let prob = r.gen_range(0..100);

    let mut t = 1.;
    if (20..40).contains(&prob) {
        t = 1.5;
    } else if (40..60).contains(&prob) {
        t = 2.;
    } else if (60..80).contains(&prob) {
        t = 2.5;
    } else if (80..100).contains(&prob) {
        t = 3.;
    }

    p.timer = Timer::from_seconds(t, TimerMode::Once);
}

impl Default for Pattern3 {
    fn default() -> Self {
        Pattern3 {
            timer: Timer::from_seconds(1., TimerMode::Once),
            dir_x: 1.,
            dir_y: 1.,
            speed: 1.,
        }
    }
}
