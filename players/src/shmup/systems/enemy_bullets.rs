use super::super::components::enemy_bullets::Bullet;
use bevy::prelude::*;
use std::time::Duration;

pub fn movement(time: Res<Time>, mut query: Query<(&mut Transform, &Bullet), With<Bullet>>) {
    for (mut transform, bullet) in query.iter_mut() {
        //move the bullet to a target position
        transform.translation.x += bullet.speed * bullet.direction.x * time.delta_seconds();
        transform.translation.y += bullet.speed * bullet.direction.y * time.delta_seconds();
    }
}

pub fn despawn(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Bullet), With<Bullet>>,
) {
    for (entity, mut bullet) in query.iter_mut() {
        if bullet.life_timer.finished() {
            commands.entity(entity).despawn();
        } else {
            bullet
                .life_timer
                .tick(Duration::from_secs_f32(time.delta_seconds()));
        }
    }
}
