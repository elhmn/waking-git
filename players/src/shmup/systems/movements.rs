use std::time::Duration;

use super::super::components::patterns;
use super::super::components::player;
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

pub fn move_towards(
    time: Res<Time>,
    mut pattern_query: Query<(&mut Transform, &mut patterns::MoveTowards)>,
    player_query: Query<&Transform, (With<player::Player>, Without<patterns::MoveTowards>)>,
) {
    for (mut pattern_transform, mut pattern) in pattern_query.iter_mut() {
        if let Ok(player) = player_query.get_single() {
            let mut direction = player.translation - pattern_transform.translation;
            direction = direction.normalize();
            pattern_transform.translation += direction * pattern.speed * time.delta_seconds();

            //Update the speed
            let speed = pattern.speed + pattern.speed * time.delta_seconds();
            if is_in_range(player.translation, pattern_transform.translation, 50.) {
                pattern.speed = pattern.initial_speed;
            } else {
                pattern.speed = if speed > pattern.max_speed {
                    pattern.max_speed
                } else {
                    speed
                };
            }
        }
    }
}

pub fn is_in_range(target: Vec3, position: Vec3, range: f32) -> bool {
    let distance = (target - position).length();
    distance < range
}
