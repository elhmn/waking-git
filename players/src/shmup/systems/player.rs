use super::super::components::camera;
use super::super::components::enemy;
use super::super::components::enemy_bullets;
use super::super::components::player;
use super::super::components::player_bullet;
use super::super::config;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use std::time::Duration;

pub fn movement(
    mut query: Query<(&mut Transform, &player::Velocity, &player::Player), With<player::Player>>,
    time: Res<Time>,
) {
    for (mut transform, velocity, player) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds() * player.speed;
        transform.translation.y += velocity.y * time.delta_seconds() * player.speed;
    }
}

pub fn keyboad_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut player::Velocity, &player::Player)>,
) {
    if let Ok((mut velocity, player)) = query.get_single_mut() {
        velocity.x = if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A)
        {
            -player.speed
        } else if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            player.speed
        } else {
            0.
        };

        velocity.y = if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            player.speed
        } else if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            -player.speed
        } else {
            0.
        };

        //We need to normalize the velocity vector
        //to avoid diagonal movement being faster
        if velocity.x != 0. && velocity.y != 0. {
            velocity.x /= 2_f32.sqrt();
            velocity.y /= 2_f32.sqrt();
        }
    }
}

pub fn mouse_input(
    mut commands: Commands,
    windows: Res<Windows>,
    mouse: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut player::Gun), With<player::Player>>,
    cam: Query<(&Camera, &GlobalTransform), With<camera::MainCamera>>,
) {
    let win = windows.primary();
    let (camera, camera_transform) = cam.single();
    if let Ok((mut transform, mut gun)) = query.get_single_mut() {
        let mouse_pos = win
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
            .unwrap_or_default();

        //Get the direction to the mouse
        let mut mouse_direction = Vec2::new(
            mouse_pos.x - transform.translation.x,
            mouse_pos.y - transform.translation.y,
        );
        mouse_direction = mouse_direction.normalize();

        //Rotate the player to face the mouse
        if !mouse_direction.is_nan() {
            transform.rotation = Quat::from_rotation_arc(Vec3::Y, mouse_direction.extend(0.));
        }

        if mouse.pressed(MouseButton::Left) {
            if gun.cooldown_timer.finished() {
                let size = Vec2::new(20., 60.);
                let padding = 20.;
                let col_sprite = SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(0., 0., -0.1)),
                    sprite: Sprite {
                        color: Color::hex(config::get_col_color()).unwrap_or_default(),
                        custom_size: Some(Vec2::new(size.x + padding, size.y + padding)),
                        ..default()
                    },
                    ..default()
                };

                commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::hex("39d353").unwrap_or_default(),
                            custom_size: Some(Vec2::new(size.x, size.y)),
                            ..default()
                        },
                        transform: Transform {
                            translation: transform.translation
                                + Vec3::new(60., 60., 1.) * mouse_direction.extend(0.),
                            //We need to rotate the bullet to face the mouse
                            rotation: Quat::from_rotation_arc(Vec3::Y, mouse_direction.extend(0.)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(col_sprite)
                            .insert(player_bullet::BulletCollider::default());
                    })
                    .insert(player_bullet::Bullet {
                        direction: mouse_direction,
                        ..Default::default()
                    });
                gun.cooldown_timer.reset();
            } else {
                gun.cooldown_timer
                    .tick(Duration::from_secs_f32(time.delta_seconds()));
            }
        }
    }
}

//naive collision detection
#[allow(clippy::type_complexity)]
pub fn player_enemy_bullets_collisions(
    mut commands: Commands,
    player_query: Query<(&Parent, &mut GlobalTransform, &mut Sprite), With<player::PlayerCollider>>,
    enemy_bullets: Query<
        (&Parent, &mut GlobalTransform, &mut Sprite),
        (
            With<enemy_bullets::BulletCollider>,
            Without<player::PlayerCollider>,
        ),
    >,
) {
    let (_, player_transform, player_sprite) = player_query.get_single().unwrap();
    for (bullet, bullet_transform, bullet_sprite) in enemy_bullets.iter() {
        //if bullet collides with the player
        //the collision is detected using the bevy::sprite::collide_aabb::collide function
        //destroy the bullet and the player
        if collide(
            player_transform.translation(),
            player_sprite.custom_size.unwrap_or_default(),
            bullet_transform.translation(),
            bullet_sprite.custom_size.unwrap_or_default(),
        )
        .is_some()
        {
            commands.entity(bullet.get()).despawn_recursive();
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn player_bullets_enemy_bullets_collisions(
    mut commands: Commands,
    player_bullets: Query<
        (&Parent, &mut GlobalTransform, &mut Sprite),
        With<player_bullet::BulletCollider>,
    >,
    enemy_bullets: Query<
        (&Parent, &mut GlobalTransform, &mut Sprite),
        (
            With<enemy_bullets::BulletCollider>,
            With<enemy_bullets::DestructibleBullet>,
            Without<player_bullet::BulletCollider>,
        ),
    >,
) {
    for (bullet, bullet_transform, bullet_sprite) in enemy_bullets.iter() {
        for (player_bullet, player_bullet_transform, player_bullet_sprite) in player_bullets.iter()
        {
            //if bullet collides with the player bullet
            //the collision is detected using the bevy::sprite::collide_aabb::collide function
            //destroy the bullet and the player bullet
            if collide(
                player_bullet_transform.translation(),
                player_bullet_sprite.custom_size.unwrap_or_default(),
                bullet_transform.translation(),
                bullet_sprite.custom_size.unwrap_or_default(),
            )
            .is_some()
            {
                commands.entity(bullet.get()).despawn_recursive();
                commands.entity(player_bullet.get()).despawn_recursive();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn player_bullets_enemies_collisions(
    mut commands: Commands,
    player_bullets: Query<
        (&Parent, &mut GlobalTransform, &mut Sprite),
        With<player_bullet::BulletCollider>,
    >,
    enemies: Query<
        (&Parent, &mut GlobalTransform, &mut Sprite),
        (
            With<enemy::EnemyCollider>,
            Without<player_bullet::BulletCollider>,
        ),
    >,
) {
    for (_, enemy_transform, enemy_sprite) in enemies.iter() {
        for (player_bullet, player_bullet_transform, player_bullet_sprite) in player_bullets.iter()
        {
            //if bullet collides with the player bullet
            //the collision is detected using the bevy::sprite::collide_aabb::collide function
            //destroy the bullet and the player bullet
            if collide(
                player_bullet_transform.translation(),
                player_bullet_sprite.custom_size.unwrap_or_default(),
                enemy_transform.translation(),
                enemy_sprite.custom_size.unwrap_or_default(),
            )
            .is_some()
            {
                //                 commands.entity(enemy.get()).despawn_recursive();
                commands.entity(player_bullet.get()).despawn_recursive();
            }
        }
    }
}
