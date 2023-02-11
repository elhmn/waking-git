use std::time::Duration;

use super::super::components::patterns;
use bevy::prelude::*;

pub fn movement_pattern_1(
    mut query: Query<(&patterns::Pattern1, &mut Transform)>,
    time: Res<Time>,
) {
    for (p, mut t) in query.iter_mut() {
        t.translation.x += 100. * p.speed * time.delta_seconds();
    }
}

pub fn movement_pattern_2(
    mut query: Query<(&patterns::Pattern2, &mut Transform)>,
    time: Res<Time>,
) {
    for (p, mut t) in query.iter_mut() {
        t.translation.x -= 100. * p.speed * time.delta_seconds();
    }
}

pub fn movement_pattern_3(
    mut query: Query<(&mut patterns::Pattern3, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut p, mut t) in query.iter_mut() {
        p.timer.tick(Duration::from_secs_f32(time.delta_seconds()));

        if p.timer.finished() {
            patterns::pattern3_random_dir(&mut p);
            //randomly determine the direction
            p.timer.reset();
        }

        t.translation.x += p.speed * p.dir_x * time.delta_seconds();
        t.translation.y += p.speed * p.dir_y * time.delta_seconds();
    }
}

pub fn pattern_3_init(mut query: Query<&mut patterns::Pattern3>) {
    for mut p in query.iter_mut() {
        patterns::pattern3_random_dir(&mut p);
        patterns::pattern3_random_time(&mut p);
    }
}
