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
    mut query: Query<(&patterns::Pattern3, &mut Transform)>,
    time: Res<Time>,
) {
    for (p, mut t) in query.iter_mut() {
        t.translation.y += 100. * p.speed * time.delta_seconds();
    }
}
